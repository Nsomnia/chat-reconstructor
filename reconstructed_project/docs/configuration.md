# Configuration

VibeStream stores its configuration in `$HOME/.config/vibestream/config.json`.

## Settings

### General Settings

- `artist_name`: The default artist name to use when no ID3 tag is present.
- `social_media_link`: The social media URL to display in the visualization.

### Text Overlay Settings

- `font_path`: Path to the font file used for text overlays.
- `font_size`: Size of the artist name and title text.
- `text_opacity`: Opacity of the artist name and title text (0.0 to 1.0).
- `text_color`: RGBA color of the artist name and title text.
- `text_move_speed`: Speed at which the artist name and title text move.
- `fade_duration_secs`: Duration of the fade in/out effect in seconds.

### Social Media Settings

- `social_media_display_duration_secs`: How long the social media link stays in one corner before moving.
- `social_media_font_size`: Size of the social media link text.
- `social_media_opacity`: Opacity of the social media link text (0.0 to 1.0).

## Example Configuration

```json
{
  "artist_name": "Example Artist",
  "social_media_link": "https://example.com",
  "font_path": "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
  "font_size": 24,
  "text_opacity": 1.0,
  "text_color": [255, 255, 255, 255],
  "text_move_speed": 100.0,
  "fade_duration_secs": 5.0,
  "social_media_display_duration_secs": 10.0,
  "social_media_font_size": 16,
  "social_media_opacity": 0.8
}
