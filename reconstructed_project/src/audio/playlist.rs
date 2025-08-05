use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
}

impl PlaylistItem {
    pub fn new(path: PathBuf) -> Self {
        let mut title = None;
        let mut artist = None;

        // Try to read ID3 tags
        if let Ok(tag) = id3::Tag::read_from_path(&path) {
            title = tag.title().map(|s| s.to_string());
            artist = tag.artist().map(|s| s.to_string());
        }

        // If no title, use filename without extension
        if title.is_none() {
            if let Some(file_stem) = path.file_stem() {
                title = file_stem.to_string_lossy().to_string().into();
            }
        }

        Self {
            path,
            title,
            artist,
        }
    }
}

pub struct Playlist {
    items: Vec<PlaylistItem>,
    current_index: Option<usize>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: None,
        }
    }

    pub fn add_item(&mut self, path: PathBuf) {
        let item = PlaylistItem::new(path);
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);

            // Adjust current index if needed
            if let Some(current) = self.current_index {
                if current == index {
                    self.current_index = None;
                } else if current > index {
                    self.current_index = Some(current - 1);
                }
            }
        }
    }

    pub fn move_item(&mut self, from_index: usize, to_index: usize) {
        if from_index < self.items.len() && to_index < self.items.len() && from_index != to_index {
            let item = self.items.remove(from_index);
            self.items.insert(to_index, item);

            // Adjust current index if needed
            if let Some(current) = self.current_index {
                if current == from_index {
                    self.current_index = Some(to_index);
                } else if from_index < current && to_index >= current {
                    self.current_index = Some(current - 1);
                } else if from_index > current && to_index <= current {
                    self.current_index = Some(current + 1);
                }
            }
        }
    }

    pub fn get_items(&self) -> &Vec<PlaylistItem> {
        &self.items
    }

    pub fn get_current_item(&self) -> Option<&PlaylistItem> {
        self.current_index.and_then(|i| self.items.get(i))
    }

    pub fn get_current_index(&self) -> Option<usize> {
        self.current_index
    }

    pub fn set_current_index(&mut self, index: Option<usize>) {
        if let Some(i) = index {
            if i < self.items.len() {
                self.current_index = Some(i);
            } else {
                self.current_index = None;
            }
        } else {
            self.current_index = None;
        }
    }

    pub fn next(&mut self) -> Option<&PlaylistItem> {
        if self.items.is_empty() {
            return None;
        }

        let next_index = match self.current_index {
            Some(i) => {
                if i + 1 < self.items.len() {
                    i + 1
                } else {
                    return None; // End of playlist
                }
            }
            None => 0,
        };

        self.current_index = Some(next_index);
        self.items.get(next_index)
    }

    pub fn previous(&mut self) -> Option<&PlaylistItem> {
        if self.items.is_empty() {
            return None;
        }

        let prev_index = match self.current_index {
            Some(0) => return None, // Beginning of playlist
            Some(i) => i - 1,
            None => self.items.len() - 1,
        };

        self.current_index = Some(prev_index);
        self.items.get(prev_index)
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
    }
}
