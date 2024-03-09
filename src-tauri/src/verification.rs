use reqwest::Client;

// Where i can get all this params without asking for it --> Fix this to put default value
pub async fn etherscan_verification(
    api_key: &str,
    contract_address: &str,
    source_code: &str,
    contract_name: &str,
    compiler_version: &str,
    optimization_used: &str,
    runs: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    //Example
    //let compiler_version = "v0.8.0+commit.c7dfd78e"; // Compiler version used
    //let optimization_used = "1"; // Whether optimization was used (0 = No, 1 = Yes)
    //let runs = "200"; // Optimization runs
    // Additional parameters like constructor arguments, library addresses, etc., may be require
    let client = Client::new();
    let res = client
        .post("https://api-sepolia.etherscan.io/api")
        .form(&[
            ("apikey", api_key),
            ("module", "contract"),
            ("action", "verifysourcecode"),
            ("contractaddress", contract_address),
            ("sourceCode", source_code),
            ("contractname", contract_name),
            ("compilerversion", compiler_version),
            ("optimizationUsed", optimization_used),
            ("runs", runs),
            // Add other form fields as needed
        ])
        .send()
        .await?;

    let response_text = res.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}

#[cfg(test)]
pub mod tests {
    
    use super::*;

    #[tokio::test]
    async fn test_etherscan_verification() {
        let api_key = "D7MEF2GFCVH4WEC69MSAIHTQ64F3U7EXMU";
        let contract_address = "0x91BD3394ce59fe635253E3739D25Af07DD4952f4";
        let file_path = "src/soliditylayout/contracts/storage.sol";

        // Asynchronously read the file contents into a string
        let source_code = tokio::fs::read_to_string(file_path)
            .await
            .expect("Failed to read source code from file");

        let contract_name = "Owner";
        let compiler_version = "v0.8.24+commit.e11b9ed9";
        let optimization_used = "0";
        let runs = "200";

        // Ensure you are awaiting the result of the async function
        let verification_result = etherscan_verification(
            api_key,
            contract_address,
            &source_code, // Pass the file contents as source_code
            contract_name,
            compiler_version,
            optimization_used,
            runs,
        )
        .await;

        // Here you might want to assert something about verification_result to make the test meaningful
        println!("Verification Result: {:?}", verification_result);

        // Example assertion (adjust according to what etherscan_verification returns and what you expect)
        // assert!(verification_result.is_ok());
    }

}
