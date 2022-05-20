use std::sync::Mutex;
use anni_repo::db::RepoDatabaseRead;
use async_graphql::{Context, Object, SimpleObject, ComplexObject};

#[derive(SimpleObject)]
#[graphql(complex)]
struct AlbumInfo {
    album_id: String,
    title: String,
    edition: Option<String>,
    catalog: String,
    artist: String,
    release_date: String,
    album_type: String,
}

#[ComplexObject]
impl AlbumInfo {
    async fn discs(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<DiscInfo>> {
        let manager = ctx.data_unchecked::<Mutex<RepoDatabaseRead>>();
        let rows = manager.lock().unwrap().get_discs(self.album_id.parse()?)?;
        Ok(rows.into_iter().map(|r| DiscInfo {
            album_id: r.album_id.0.to_string(),
            disc_id: r.disc_id,
            title: r.title,
            artist: r.artist,
            catalog: r.catalog,
            disc_type: r.disc_type,
        }).collect())
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct DiscInfo {
    album_id: String,
    disc_id: u8,

    title: String,
    artist: String,
    catalog: String,
    disc_type: String,
}

#[ComplexObject]
impl DiscInfo {
    async fn tracks(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<TrackInfo>> {
        let manager = ctx.data_unchecked::<Mutex<RepoDatabaseRead>>();
        let rows = manager.lock().unwrap().get_tracks(self.album_id.parse()?, self.disc_id)?;
        Ok(rows.into_iter().map(|r| TrackInfo {
            album_id: r.album_id.0.to_string(),
            disc_id: r.disc_id,
            track_id: r.track_id,
            title: r.title,
            artist: r.artist,
            track_type: r.track_type,
        }).collect())
    }

    async fn track(&self, ctx: &Context<'_>, track_id: u8) -> anyhow::Result<Option<TrackInfo>> {
        let manager = ctx.data_unchecked::<Mutex<RepoDatabaseRead>>();
        let row = manager.lock().unwrap().get_track(self.album_id.parse()?, self.disc_id, track_id)?;
        Ok(row.map(|r| TrackInfo {
            album_id: r.album_id.0.to_string(),
            disc_id: r.disc_id,
            track_id: r.track_id,
            title: r.title,
            artist: r.artist,
            track_type: r.track_type,
        }))
    }
}

#[derive(SimpleObject)]
struct TrackInfo {
    album_id: String,
    disc_id: u8,
    track_id: u8,

    title: String,
    artist: String,
    track_type: String,
}

pub struct AnnivQuery;

#[Object]
impl AnnivQuery {
    async fn album(&self, ctx: &Context<'_>, album_id: String) -> anyhow::Result<AlbumInfo> {
        let manager = ctx.data_unchecked::<Mutex<RepoDatabaseRead>>();
        let row = manager.lock().unwrap().get_album(album_id.parse()?)?.unwrap();
        Ok(AlbumInfo {
            album_id: row.album_id.0.to_string(),
            title: row.title,
            edition: row.edition,
            catalog: row.catalog,
            artist: row.artist,
            release_date: row.release_date,
            album_type: row.album_type,
        })
    }

    // async fn disc(&self, album_id: String, disc_id: u8) -> DiscInfo {
    //     DiscInfo { name }
    // }
    //
    // async fn playlist(&self, id: String) -> PlaylistInfo {
    //     PlaylistInfo::new(id.0)
    // }
}