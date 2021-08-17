use std::{convert::TryInto, time::Duration};

pub mod tests_common;
use crypto::{ToBase58Check, hex};
use tests_common::{build_sandbox_http_apis, sandbox_account_1, sandbox_account_2};

use rpc_api::api::*;
use explorer_api::TzStats;
use signer::LocalSigner;
use types::{Address, BlockHash, Forge, Forged, Network, NewOperationGroup, NewOriginationOperation, NewOriginationScript, NewTransactionOperation, NewTransactionParameters, micheline::{Micheline, MichelineEntrypoint, MichelinePrim, PrimType}};
use utils::parse_float_amount;

pub mod zkchannels;
use zkchannels::{samples, contract_code_forged};

const BLOCK_LEN: i32 = 20;

async fn contract_origination_helper(port: u32, cust_funding: String, merch_funding: String) -> (String, String) {
    let (_, async_api) = build_sandbox_http_apis(port);
    let cust_account = sandbox_account_1();
    let merch_account = sandbox_account_2();

    let signer = LocalSigner::new(cust_account.public_key.clone(), cust_account.private_key.clone());
    let cust_addr = cust_account.address.to_base58check();
    let merch_addr = merch_account.address.to_base58check();

    let op = NewOriginationOperation {
        source: cust_account.address.clone().into(),
        script: NewOriginationScript {
            code: contract_code_forged(),
            storage: samples::sample1::initial_storage(cust_addr, merch_addr, cust_funding, merch_funding).into(),
        },
        balance: parse_float_amount("0.01").unwrap(),
        fee: parse_float_amount("0.1").unwrap(),
        counter: async_api.get_contract_counter(&cust_account.address).await.unwrap() + 1,
        gas_limit: 50000,
        storage_limit: 20000,
    };

    let operation_group = NewOperationGroup::new(
        async_api.get_head_block_hash().await.unwrap(),
        async_api.get_protocol_info().await.unwrap().next_protocol_hash,
    ).with_operation(op);

    let forged = operation_group.forge();
    let signed = signer.sign_forged_operation_bytes(forged.as_ref());
    let op_hash = signed.operation_hash;

    dbg!(async_api.inject_operations(&signed.operation_with_signature).await.unwrap());
    assert_eq!(
        async_api.get_pending_operation_status(&op_hash).await.unwrap(),
        PendingOperationStatus::Applied,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();

        if let PendingOperationStatus::Finished = status {
            break;
        }
    }

    // get the latest block hash and search until we find the contract address with operation hash
    let latest_block_hash = async_api.get_head_block_hash().await.unwrap();
    // println!("[*] latest block hash: {:?}", latest_block_hash);

    let mut contract_address = String::from("");
    let block_operations = async_api.block_get_operations(&latest_block_hash).await.unwrap();
    assert!(block_operations.len() >= 1);
    let mut op_count = 0;
    for o in block_operations {
        op_count += 1;
        // println!("{} => op hash: {:?}", i, o.hash.to_base58check());
        // println!("{} => branch hash: {:?}", i, o.branch.to_base58check());
        for c in o.contents {
            match c {
                BlockOperationContent::Origination(op) => {

                    for contract in &op.metadata.operation_result.originated_contracts {
                        assert_eq!(o.hash.to_base58check(), op_hash);
                        contract_address = contract.to_base58check();
                        break;
                    }
                },
                BlockOperationContent::Reveal(_) => {},
                BlockOperationContent::Transaction(_) => {},
                BlockOperationContent::Delegation(_) => {},
                BlockOperationContent::Other => {},
            }
        }
    }
    
    (op_hash, contract_address)
}

// TODO: WIP due to missing functionality
async fn search_transaction_helper(port: u32, op_hash: String, contract_address: String) {
    let (_, async_api) = build_sandbox_http_apis(port);

    let mut current_block_hash = async_api.get_head_block_hash().await.unwrap();

    // search up to some constant number of blocks from latest block
    let mut block_count = 0;
    loop {
        let block_operations = async_api.block_get_operations(&current_block_hash).await.unwrap();
        assert!(block_operations.len() >= 1);
        let mut op_count = 0;
        for o in block_operations {
            op_count += 1;
            for c in o.contents {
                match c {
                    BlockOperationContent::Origination(_) => {},
                    BlockOperationContent::Reveal(_) => {},
                    BlockOperationContent::Transaction(op) => {
                        println!("found a transaction: {:?}", &o.hash);
                    },
                    BlockOperationContent::Delegation(_) => {},
                    BlockOperationContent::Other => {},
                }
            }
        }
        block_count += 1;
        // TODO: how do I get block hash of the previous block?

        // exit after 20 blocks
        if block_count > BLOCK_LEN {
            break;
        }
    }

}

async fn add_funding(port: u32, contract_address: String, amount: u64, is_cust: bool) -> String {
    let (_, async_api) = build_sandbox_http_apis(port);

    // // initiate transaction to fund the account
    let funding_account = match is_cust {
        true => sandbox_account_1(),
        false => sandbox_account_2()
    };
    let signer = LocalSigner::new(funding_account.public_key.clone(), funding_account.private_key.clone());

    // now proceed with depositing funds into the contract
    let tx = NewTransactionOperation {
        source: funding_account.address.clone().into(),
        destination: Address::from_base58check( contract_address.as_str()).unwrap(),
        amount: amount,
        fee: parse_float_amount("0.01").unwrap(),
        counter: async_api.get_contract_counter(&funding_account.address).await.unwrap() + 1,
        gas_limit: 50000,
        storage_limit: 20000,
        parameters: Some(NewTransactionParameters::Custom {
            entrypoint: MichelineEntrypoint::Custom("addFunding".to_owned()),
            data: MichelinePrim::new(PrimType::None).into(),
        })
    };

    let operation_group = NewOperationGroup::new(
        async_api.get_head_block_hash().await.unwrap(),
        async_api.get_protocol_info().await.unwrap().next_protocol_hash,
    ).with_transaction(tx);

    let forged = operation_group.forge();
    let signed = signer.sign_forged_operation_bytes(forged.as_ref());
    let op_hash = signed.operation_hash;
    async_api.inject_operations(&signed.operation_with_signature).await.unwrap();
    assert_eq!(
        async_api.get_pending_operation_status(&op_hash).await.unwrap(),
        PendingOperationStatus::Finished,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();
        if let PendingOperationStatus::Finished = status {
            break;
        }
    }
    
    op_hash
}

async fn reclaim_funding(port: u32, contract_address: String, amount: u64) -> String {
    let (_, async_api) = build_sandbox_http_apis(port);

    let cust_account = sandbox_account_1();
    let signer = LocalSigner::new(cust_account.public_key.clone(), cust_account.private_key.clone());

    // now proceed with depositing funds into the contract
    let tx = NewTransactionOperation {
        source: cust_account.address.clone().into(),
        destination: Address::from_base58check( contract_address.as_str()).unwrap(),
        amount: amount,
        fee: parse_float_amount("0.01").unwrap(),
        counter: async_api.get_contract_counter(&cust_account.address).await.unwrap() + 1,
        gas_limit: 50000,
        storage_limit: 20000,
        parameters: Some(NewTransactionParameters::Custom {
            entrypoint: MichelineEntrypoint::Custom("reclaimFunding".to_owned()),
            data: MichelinePrim::new(PrimType::None).into(),
        })
    };

    let operation_group = NewOperationGroup::new(
        async_api.get_head_block_hash().await.unwrap(),
        async_api.get_protocol_info().await.unwrap().next_protocol_hash,
    ).with_transaction(tx);

    let forged = operation_group.forge();
    let signed = signer.sign_forged_operation_bytes(forged.as_ref());
    let op_hash = signed.operation_hash;
    async_api.inject_operations(&signed.operation_with_signature).await.unwrap();
    assert_eq!(
        async_api.get_pending_operation_status(&op_hash).await.unwrap(),
        PendingOperationStatus::Finished,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();
        if let PendingOperationStatus::Finished = status {
            break;
        }
    }
    
    op_hash
}

async fn cust_close(port: u32, contract_address: String, cust_bal: String, merch_bal: String, rev_lock: String, sigma1: String, sigma2: String) -> String {
    let (_, async_api) = build_sandbox_http_apis(port);

    let cust_account = sandbox_account_1();
    let signer = LocalSigner::new(cust_account.public_key.clone(), cust_account.private_key.clone());

    // now proceed with cust close call
    let tx = NewTransactionOperation {
        source: cust_account.address.clone().into(),
        destination: Address::from_base58check( contract_address.as_str()).unwrap(),
        amount: parse_float_amount("0").unwrap(),
        fee: parse_float_amount("0.01").unwrap(),
        counter: async_api.get_contract_counter(&cust_account.address).await.unwrap() + 1,
        gas_limit: 50000,
        storage_limit: 20000,
        parameters: Some(NewTransactionParameters::Custom {
            entrypoint: MichelineEntrypoint::Custom("custClose".to_owned()),
            data: MichelinePrim::new(PrimType::Pair)
            .with_args( vec![   
                Micheline::Int(cust_bal.parse().unwrap()),
                Micheline::Int(merch_bal.parse().unwrap()),
                Micheline::Bytes(hex::decode(rev_lock).unwrap()),
                Micheline::Bytes(hex::decode(sigma1).unwrap()),
                Micheline::Bytes(hex::decode(sigma2).unwrap()),
            ]).into(), // TODO: format the args correctly
        })
    };

    let operation_group = NewOperationGroup::new(
        async_api.get_head_block_hash().await.unwrap(),
        async_api.get_protocol_info().await.unwrap().next_protocol_hash,
    ).with_transaction(tx);

    let forged = operation_group.forge();
    let signed = signer.sign_forged_operation_bytes(forged.as_ref());
    let op_hash = signed.operation_hash;
    async_api.inject_operations(&signed.operation_with_signature).await.unwrap();
    assert_eq!(
        async_api.get_pending_operation_status(&op_hash).await.unwrap(),
        PendingOperationStatus::Finished,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();
        if let PendingOperationStatus::Finished = status {
            break;
        }
    }
    
    op_hash
}


#[tokio::test]
async fn test_zkchannels_contract_sandbox() {
    let (_, async_api) = build_sandbox_http_apis(20000);
    let cust_account = sandbox_account_1();
    let merch_account = sandbox_account_2();

    let signer = LocalSigner::new(cust_account.public_key.clone(), cust_account.private_key.clone());
    let cust_addr = cust_account.address.to_base58check();
    let merch_addr = merch_account.address.to_base58check();
    let cust_funding = String::from("20000000");
    let merch_funding = String::from("10000000");

    let op = NewOriginationOperation {
        source: cust_account.address.clone().into(),
        script: NewOriginationScript {
            code: contract_code_forged(),
            storage: samples::sample1::initial_storage(cust_addr, merch_addr, cust_funding, merch_funding).into(),
        },
        balance: parse_float_amount("0.01").unwrap(),
        fee: parse_float_amount("0.1").unwrap(),
        counter: async_api.get_contract_counter(&cust_account.address).await.unwrap() + 1,
        gas_limit: 50000,
        storage_limit: 20000,
    };

    let operation_group = NewOperationGroup::new(
        async_api.get_head_block_hash().await.unwrap(),
        async_api.get_protocol_info().await.unwrap().next_protocol_hash,
    ).with_operation(op);

    let forged = operation_group.forge();
    let signed = signer.sign_forged_operation_bytes(forged.as_ref());
    let op_hash = signed.operation_hash;

    dbg!(async_api.inject_operations(&signed.operation_with_signature).await.unwrap());
    assert_eq!(
        async_api.get_pending_operation_status(&op_hash).await.unwrap(),
        PendingOperationStatus::Applied,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();

        if let PendingOperationStatus::Finished = status {
            break;
        }
    }

    println!("contract successfuly originated: {}", op_hash);
}

#[tokio::test]
async fn test_add_funding_transaction_sandbox() {
    let port = 20000;
    let cust_funding = String::from("30000000");
    let merch_funding = String::from("10000000");
    let (contract_op_hash, contract_address) = contract_origination_helper(port, cust_funding, merch_funding).await;
    println!("contract successfuly originated: {} => {}", contract_op_hash, contract_address);

    let op_hash1 = add_funding(port, contract_address.clone(), parse_float_amount("30.0").unwrap(), true).await;
    println!("customer adds funding to contract. op hash: {}", op_hash1);

    let op_hash2 = add_funding(port, contract_address.clone(), parse_float_amount("10.0").unwrap(), false).await;
    println!("merchant adds funding to contract. op hash: {}", op_hash2);

    // TODO: use correct arguments
    let cust_bal = String::from("19800000");
    let merch_bal = String::from("10200000");
    let rev_lock = String::from("43e42af9f7a6c1a91805a9d9b393f7a40d027a606e3d82e5e17ea886b4c20c46");
    let sigma1 = String::from("11429a20a1f19580c9e21d85ef96e9b332ddd25db718200fedae3fc5c9193596dc25326a1e16be04aca3528a4f7cac42057842827e352db32f5efa008f6bd7b55465368e2e9b2791951888cf2a2d78c87bf7244843b84f5baa383921c83a5cba");
    let sigma2 = String::from("09f67794d6a39d05abcb33b1105b9db6b6e3bdf6fdc1d55c24e2f8ce31c986f78d2b76eb054c4a35c12b1ff1f073a2a6134237940cd328a8a14fb55935e9bb1c5a44320b37f9b2d55050bf5555ee255f750553442fc19b78b89fe373ebccee3d");
    let op_hash3 = cust_close(port, contract_address.clone(), cust_bal, merch_bal, rev_lock, sigma1, sigma2).await;
    println!("calling cust close in the contract. op hash: {}", op_hash3);

    search_transaction_helper(port, op_hash1, contract_address).await;
}

