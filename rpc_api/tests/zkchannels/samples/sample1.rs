use crate::zkchannels::InitialStorage;

pub fn initial_storage(
    cust_addr: String,
    merch_addr: String,
    cust_funding: String,
    merch_funding: String,
) -> InitialStorage {
    InitialStorage {
        // cid: String::from("0x49e2bf68c90fdb873853320c6e5a7ec5bd00b72e17e6cd96c1de19b0e9652d4b"),
        cid: String::from("49e2bf68c90fdb873853320c6e5a7ec5bd00b72e17e6cd96c1de19b0e9652d4b"),

        // close_flag: String::from("0x1a5dd750f1dac8698369bcede9f13ba6c99ecfced8d5ac366219c245d24f4b36"),
        close_flag: String::from("1a5dd750f1dac8698369bcede9f13ba6c99ecfced8d5ac366219c245d24f4b36"),

        context_string: String::from("49e2bf68c90fdb873853320c6e5a7ec5bd00b72e17e6cd96c1de19b0e9652d4b"),
        custAddr: cust_addr, // String::from("tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx"),
        custBal: String::from("0"),
        custFunding: cust_funding, // String::from("20000000"),
        custPk: String::from("edpku5Ei6Dni4qwoJGqXJs13xHfyu4fhUg6zqZkFyiEh1mQhFD3iZE"),
        delayExpiry: String::from("0"),
        g2: String::from("12ade57fe34fbe7a6fdcb1fc0d828cb3a5ef7fd346f5ea5cbea3b93e4514fae09d674b4d66d3bc673c4f831c8e24b8780fffb940d1776bfa796992c6d8f3d1a009394bbaf590fa2997ff97ba4c8dca3df4a2fc1d8059c2ccf914322823d870b00770ad29db057f19b894748ea2b1b622c00d94d5a412d61c7a0797f6d5c7b5d22e7bffd3f6f87158105020f9c625941b055c2555b5dcefc3c1b40f1098a3546e655e91c94ceb5db3f3ecd405caf39b2dda56412ebf3796e54043b0cc8d30558b"),
        merchAddr: merch_addr, // String::from("tz1gjaF81ZRRvdzjobyfVNsAeSC6PScjfQwN"),
        merchBal: String::from("0"),
        merchFunding: merch_funding, // String::from("10000000"),
        merchPk: String::from("edpkuQd8M26sz4ynC7nfer8J2CH5fou9eseszHbdGMLz3e1WSj9cmR"),
        merchPk0: String::from("189d6846b9a2bfada602de7ebc71aa26e0ad4843bd84ced29d8ca7018978ab8e616a38bd5f23038b8c27e20d99390f4200742ab26fe59700aa9ecbfa035511c57af541a9166641088a47d09338811aecaaa399e0c95d6d8e422b318f68fac3b812808658af18177e7f3198e15279e66eebb2c5638d8c1f8a2683174fb21ae70504a1ebd3590d4f65e292c09c7b52abe810c139a8fa243314fa60922d528b240d03d2e7714a47ae3fb8999cbae79c9a0bfc3a1ed1d6cd0d313285ab29ce297087"),
        merchPk1: String::from("03624627ed9666b0a5be2789b9c9b5853d8d5cbd42ceb2159a439d83051676c63ae1fe8e7d484cdae6990cfbf61cfd6b12797a845850d7ed720f918929c8808abe9be8b21083e851d5c5c76c8988fe33c7ef6f56626262e8f2981fea3eca9c79095c0ab2f8ec415567309c89b31822467eb89f0b6005ce888da1fa9a6486ae6bb22dd5c33c81de51ae9d4b00e54ab75b01d7dab85e39a65bde59380b8ed0603dd8256677bc18f595e79a632df8bec510730c966db477313a1a6d2b581ae1700e"),
        merchPk2: String::from("01fee6b4807855ec81e09dcd9bf44fb0eb0303ffd2430779eeb351d83c52a4e073fe50f819f57dcf13b72319b4eff7a204d16de139291709f167d5fe87fbc6b8fbeeb4583118f024b75e613ff30f59b7bd0476cdef46e2f08e07cfb217f1747f180893fa0a3db549b6ec7c2c0d1f7a905c35d70c6442bdbae302c9bc0af7e6041c4f12edae5363854880e43dacbe896506196b7b32c03d1e8440137368aef2e6028da972e57c2eebb3020feb6c0997ac5a2f5c482f67ea36222287d19eedef1d"),
        merchPk3: String::from("0c0059043d9805bb179d241f4a9e92b71ffee88a2abf618b8a2fc4bdb7f60a892d47aca80527f217f0fc80184523b4911056fd8c116be111df38e4b606ddd0acfe8bc6ac252916d8f62cb739d6fb92f3ce67fe832a3d81c18580cf33223f36590a3ffb4500e2857e45726b0ac1e55fb162c61f71c4b6530272498a29b7b1b4762b1f05a58a079362886eb1bc4e6f22831861849ba23abc32cc1eb098cf75ee2588367f998e2c7bb9e3789980d424663978f2999578094e6e0ad5c788dbdc75a5"),
        merchPk4: String::from("051678c8a430375dc1782e41ae333f44d005961c8bffbcc0262bf0b42691ac2538fe7268b1eb37b253ee1848969c3f60142237a81dc49be3cd02cc3c461436840f70383b8742ac2b9f41715ddb6ea557de34f8edcf54c2fcfd8e2fcab78f8060175c94f077627d826aebcd44b22ec22004ecffc1c7a2dfa7b7ff510110df78df6f8f17c38346e07022d7073febb339c6082d97caf8ed1c50cd6d28a15752f72b296bb21c614bf7c9c0ce17639bce60b274596df53e08eec16f441e75a243626e"),
        merchPk5: String::from("186320ca37e72d3d54d8dec102289123afd9ff4c754b381f70fbedd44dad172ebe138f904b4ab75cf04073e19a43896f02c89b5bdae1b4ae12897697631c5e8cbafd2561b87b26546b899fb19f31a3421e6a4a16287ce1c66b62338e656fa2511812a67080cad41a0d63a96c88adef5bb365f893f056795468548fdaa09158a51861d82e15533804b7fde7f4940730cd12e39b220a07bef5a581f9612a3f70d57a9e63cf45b1ce58ec0f6491b8e8ef5571fe2583e8b08c67aedd632fcc4d2868"),
        revLock: String::from("00"),
        selfDelay: String::from("3"),
        status: String::from("0")
    }
}
