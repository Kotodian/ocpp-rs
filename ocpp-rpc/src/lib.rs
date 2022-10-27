pub mod ocpp2_0_1;
use ocpp2_0_1::json::enums::Payload;

#[macro_export]
macro_rules! feature {
    ($action: ident, $name: ident, $request: ident, $response: ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
        #[serde(untagged)]
        pub enum $name {
            Request($request),
            Response($response),
        }
        impl Into<Payload> for $request {
            fn into(self) -> Payload {
                Payload::$action($name::Request(self))
            }
        }

        impl Into<Payload> for $response {
            fn into(self) -> Payload {
                Payload::$action($name::Response(self))
            }
        }
    };
}
