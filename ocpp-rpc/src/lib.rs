pub mod ocpp2_0_1;

#[macro_export]
macro_rules! feature {
    ($name: ident, $request: ident, $response: ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
        #[serde(untagged)]
        pub enum $name {
            Request($request),
            Response($response),
        }
    };
}
