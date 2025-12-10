 let mut hash_map = HashMap::new();
    hash_map.insert(
        "d18a9a9d-6795-4f70-a5fd-1f0a78d7d138",
        "d90ad92f-6555-4f24-83e9-c8ecf312a373",
    );
    hash_map.insert(
        "a82e47cc-eb31-4eb1-806b-6993462909b1",
        "7e971cb9-12da-4556-a00c-4a51a14a47c9",
    );
    hash_map.insert(
        "fa1fcf85-9aad-4cf7-ae0c-2b31e75db6b7",
        "2a60faca-f915-4816-8f60-cf794427e3d4",
    );
    hash_map.insert(
        "1a18d0c8-9c27-43fb-ae80-a9987ab9939f",
        "2542537e-96ab-4323-ad19-b7454f5d2112",
    );
    hash_map.insert(
        "d20e0c63-196c-4568-9ab5-00989a689b50",
        "e30389e9-89c2-42ce-850a-60e10aeaabc5",
    );
    hash_map.insert(
        "850eb508-19e2-40d9-9261-10e58c17fe1f",
        "14514237-3bcd-47cb-906e-c6fc4fb36097",
    );
    hash_map.insert(
        "e8ab5a6b-0674-4ec4-bc95-3dcb336cf84e",
        "80113096-726b-40b8-9eb3-2bfc3d927f65",
    );
    hash_map.insert(
        "6a007982-1a4b-47f9-a4e3-2d650b724cf2",
        "adfc102c-30bc-4792-ab48-59e749c4f609",
    );
    hash_map.insert(
        "5056046d-8ee8-4cf8-9214-3911ad9174bb",
        "98a2db66-ba4d-41ed-be2f-d22dc89b5557",
    );

    for (key, value) in hash_map {
        let response = create_invoice
            .create_invoice_with_factory(key.to_string(), value.to_string())
            .await?;
        println!("response: {:#?}", response);
        tokio::time::sleep(time::Duration::from_secs(12)).await;
    }
