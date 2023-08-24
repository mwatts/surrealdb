use crate::api::engine::local::Db;
use crate::api::engine::local::IndxDb;
use crate::api::err::Error;
use crate::api::opt::Config;
use crate::api::opt::Endpoint;
use crate::api::opt::IntoEndpoint;
use crate::api::Result;
use url::Url;

macro_rules! endpoints {
	($($name:ty),*) => {
		$(
			impl IntoEndpoint<IndxDb> for $name {
				type Client = Db;

				fn into_endpoint(self) -> Result<Endpoint> {
					let url = format!("indxdb://{self}");
					Ok(Endpoint {
						endpoint: Url::parse(&url).map_err(|_| Error::InvalidUrl(url))?,
						config: Default::default(),
					})
				}
			}

			impl IntoEndpoint<IndxDb> for ($name, Config) {
				type Client = Db;

				fn into_endpoint(self) -> Result<Endpoint> {
					let mut endpoint = IntoEndpoint::<IndxDb>::into_endpoint(self.0)?;
					endpoint.config = self.1;
					Ok(endpoint)
				}
			}
		)*
	};
}

endpoints!(&str, &String, String);
