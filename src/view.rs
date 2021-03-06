use std::ops::{Deref, DerefMut};

use derive_more::From;

use crate::meta::{Album, Artist, Track};

#[derive(Debug, Clone, Default)]
pub struct MainView {
    insert_buffer: String,
    view: View,
}

impl MainView {
    pub fn replace_view(&mut self, view: View) -> View {
        std::mem::replace(&mut self.view, view)
    }

    pub fn insert_buffer(&self) -> &str {
        &self.insert_buffer
    }

    pub fn insert_buffer_mut(&mut self) -> &mut String {
        &mut self.insert_buffer
    }

    pub fn view_and_buffer_mut(&mut self) -> (&mut View, &mut String) {
        (&mut self.view, &mut self.insert_buffer)
    }

    pub fn view(&self) -> &View {
        &self.view
    }
}

impl Deref for MainView {
    type Target = View;

    fn deref(&self) -> &Self::Target {
        self.view()
    }
}

impl DerefMut for MainView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArtistSearch {
    pub cached_artists: Vec<Artist>,
    pub cursor: usize,
}

impl From<Vec<Artist>> for ArtistSearch {
    fn from(artists: Vec<Artist>) -> Self {
        Self {
            cached_artists: artists,
            cursor: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AlbumSearch {
    pub cached_albums: Vec<Album>,
    pub cursor: usize,
}

impl From<Vec<Album>> for AlbumSearch {
    fn from(albums: Vec<Album>) -> Self {
        Self {
            cached_albums: albums,
            cursor: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TrackList {
    pub cached_tracks: Vec<Track>,
    pub cursor: usize,
}

impl From<Vec<Track>> for TrackList {
    fn from(tracks: Vec<Track>) -> Self {
        Self {
            cached_tracks: tracks,
            cursor: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current: usize,
}

impl Playlist {
    pub fn create(tracks: Vec<Track>, current: usize) -> Self {
        Self { tracks, current }
    }
}

#[derive(Debug, Clone, From)]
pub enum View {
    ArtistSearch(ArtistSearch),
    AlbumSearch(AlbumSearch),
    TrackList(TrackList),
    Playlist(Playlist),
}

impl Default for View {
    fn default() -> Self {
        View::AlbumSearch(AlbumSearch::from(vec![]))
    }
}

pub struct CursorMut<'a> {
    cursor: &'a mut usize,
    max_cursor: usize,
}

impl Drop for CursorMut<'_> {
    fn drop(&mut self) {
        *self.cursor = self.max_cursor.min(*self.cursor);
    }
}

impl Deref for CursorMut<'_> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl DerefMut for CursorMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

#[allow(unused)]
impl View {
    pub fn name(&self) -> &'static str {
        match self {
            View::ArtistSearch(_) => "ArtistSearch",
            View::AlbumSearch(_) => "AlbumSearch",
            View::TrackList(_) => "TrackList",
            View::Playlist(_) => "Playlist",
        }
    }

    pub fn cursor(&self) -> Option<usize> {
        match self {
            View::ArtistSearch(search) => Some(search.cursor),
            View::AlbumSearch(search) => Some(search.cursor),
            View::TrackList(search) => Some(search.cursor),
            View::Playlist(_) => None,
        }
    }

    pub fn cursor_mut(&mut self) -> Option<CursorMut<'_>> {
        let max_cursor = self.len().saturating_sub(1);

        match self {
            View::ArtistSearch(search) => Some(CursorMut {
                cursor: &mut search.cursor,
                max_cursor,
            }),
            View::AlbumSearch(search) => Some(CursorMut {
                cursor: &mut search.cursor,
                max_cursor,
            }),
            View::TrackList(search) => Some(CursorMut {
                cursor: &mut search.cursor,
                max_cursor,
            }),
            View::Playlist(_) => None,
        }
    }

    pub fn reset_cursor(&mut self) {
        if let Some(mut cursor) = self.cursor_mut() {
            *cursor = 0;
        }
    }

    pub fn len(&self) -> usize {
        match self {
            View::ArtistSearch(search) => search.cached_artists.len(),
            View::AlbumSearch(search) => search.cached_albums.len(),
            View::TrackList(search) => search.cached_tracks.len(),
            View::Playlist(_) => 0,
        }
    }
}
