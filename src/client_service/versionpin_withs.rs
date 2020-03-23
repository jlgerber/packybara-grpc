use super::*;

pub mod get_versionpin_withs {
    use super::*;

    pub async fn cmd(
        grpc_client: &mut ClientService,
        versionpin_id: i64,
    ) -> Result<Vec<FindAllWithsRow>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(VersionPinWithsQueryRequest { versionpin_id });
        let response = grpc_client.client.get_version_pin_withs(request).await?;
        let VersionPinWithsQueryReply { withs } = response.into_inner();
        let withs = withs
            .into_iter()
            .map(|vpin| {
                let VersionPinWithsQueryRow {
                    id,
                    vpin_id,
                    with,
                    order,
                } = vpin;
                FindAllWithsRow::from_parts(id as i32, vpin_id as i32, with, order as i32)
            })
            .collect::<Vec<FindAllWithsRow>>();
        Ok(withs)
    }
}
