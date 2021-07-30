use std::{convert::TryInto, time::Duration};

pub mod tests_common;
use crypto::ToBase58Check;
use tests_common::{build_sandbox_http_apis, sandbox_account_1, sandbox_account_2};

use rpc_api::api::*;
use explorer_api::TzStats;
use signer::LocalSigner;
use types::{Address, Forge, Forged, Network, NewOperationGroup, NewOriginationOperation, NewOriginationScript, NewTransactionOperation, NewTransactionParameters, micheline::{Micheline, MichelineEntrypoint, MichelinePrim, PrimType}};
use utils::parse_float_amount;

pub mod zkchannels;
use zkchannels::{samples, contract_code_forged};

async fn contract_origination_helper(cust_funding: String, merch_funding: String) -> String {
    let (_, async_api) = build_sandbox_http_apis(18731);
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

    op_hash
}


#[tokio::test]
async fn test_zkchannels_contract_sandbox() {
    let (_, async_api) = build_sandbox_http_apis(18731);
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
    // let cust_funding = String::from("30000000");
    // let merch_funding = String::from("10000000");
    // let op_hash = contract_origination_helper(cust_funding, merch_funding);
    // println!("contract successfuly originated: {}", op_hash.await);
    // TODO: get contract id from the op_hash

    // initiate transaction to fund the account
    let (_, async_api) = build_sandbox_http_apis(18731);
    let cust_account = sandbox_account_1();
    let signer = LocalSigner::new(cust_account.public_key.clone(), cust_account.private_key.clone());

    // now proceed with depositing funds into the contract
    let tx = NewTransactionOperation {
        source: cust_account.address.clone().into(),
        destination: Address::from_base58check( "KT1WQiTwEtv3i48vxZdUrUWHs4vr4EeP5SVE").unwrap(),
        amount: parse_float_amount("30.0").unwrap(),
        fee: parse_float_amount("0.01").unwrap(),
        counter: async_api.get_contract_counter(&cust_account.address).await.unwrap() + 1,
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
        PendingOperationStatus::Applied,
    );

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = async_api.get_pending_operation_status(&op_hash).await.unwrap();
        if let PendingOperationStatus::Finished = status {
            break;
        }
    }
    
    println!("transaction successfuly created: {}", op_hash);
}