pub mod json;
use json::enums::Payload;

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

        impl TryFrom<Payload> for $request {
            type Error = &'static str;

            fn try_from(value: Payload) -> Result<Self, Self::Error> {
                match value {
                    Payload::$action(feature) => match feature {
                        $name::Request(request) => Ok(request),
                        _ => Err("unexpected"),
                    },
                    _ => Err("unexpected"),
                }
            }
        }

        impl TryFrom<Payload> for $response {
            type Error = &'static str;

            fn try_from(value: Payload) -> Result<Self, Self::Error> {
                match value {
                    Payload::$action(feature) => match feature {
                        $name::Response(response) => Ok(response),
                        _ => Err("unexpected"),
                    },
                    _ => Err("unexpected"),
                }
            }
        }
    };
}
