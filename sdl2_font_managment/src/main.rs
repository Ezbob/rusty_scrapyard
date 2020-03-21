extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::*;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

struct TextureWithInfo<'a> {
    texture: sdl2::render::Texture<'a>,
    queried: sdl2::render::TextureQuery,
}

impl<'a> TextureWithInfo<'a> {
    fn new_from(t: &'a TextureCreator<WindowContext>, surf: &Surface) -> TextureWithInfo<'a> {
        let b = t.create_texture_from_surface(surf).unwrap();
        let q = b.query();
        TextureWithInfo {
            texture: b,
            queried: q,
        }
    }

    fn get_texture_ref(&self) -> &Texture {
        &self.texture
    }

    fn get_texture_info_ref(&self) -> &TextureQuery {
        &self.queried
    }
}

struct TextureManager<'a> {
    font_creator: &'a TextureCreator<WindowContext>,
    cache: HashMap<usize, TextureWithInfo<'a>>,
}

impl<'a> TextureManager<'a> {
    fn new(font_creator: &'a TextureCreator<WindowContext>) -> TextureManager<'a> {
        TextureManager {
            font_creator,
            cache: HashMap::new(),
        }
    }

    fn insert_surface(&mut self, i: usize, surf: Surface) {
        self.cache
            .insert(i, TextureWithInfo::new_from(&self.font_creator, &surf));
    }

    fn get_texture(&self, i: usize) -> Option<&TextureWithInfo<'a>> {
        self.cache.get(&i)
    }
}

struct FontWithInfo<'ttf> {
    font: sdl2::ttf::Font<'ttf, 'static>,
    size: u16,
}

impl<'ttf> FontWithInfo<'ttf> {
    fn load<P: AsRef<Path>>(
        ttf: &'ttf Sdl2TtfContext,
        path: P,
        size: u16,
    ) -> Result<FontWithInfo<'ttf>, String> {
        let font = ttf.load_font(path, size).map_err(|e| e.to_string())?;
        Ok(FontWithInfo { font, size })
    }
}

struct FontCollection<'ttf> {
    b612_regular: HashMap<u16, FontWithInfo<'ttf>>,
    vt323_regular: HashMap<u16, FontWithInfo<'ttf>>,
    share_tech_mono_regular: HashMap<u16, FontWithInfo<'ttf>>
}

macro_rules! font_sizes_map {
    ($ttf: expr, $path: expr, $( $key: literal ),+) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, FontWithInfo::load($ttf, $path, $key)?); )+
         map
    }}
}

impl<'ttf> FontCollection<'ttf> {
    fn new(ttf: &'ttf Sdl2TtfContext) -> Result<FontCollection<'ttf>, String> {
        let stm_path = PathBuf::from("assets/Share_Tech_Mono/ShareTechMono-Regular.ttf");
        let b_path = PathBuf::from("assets/B612_Mono/B612Mono-Regular.ttf");
        let vt_path = PathBuf::from("assets/VT323/VT323-Regular.ttf");

        Ok(FontCollection {
            b612_regular: font_sizes_map!(ttf, &b_path, 18, 24, 30, 42),
            vt323_regular: font_sizes_map!(ttf, &vt_path, 18, 24, 30),
            share_tech_mono_regular: font_sizes_map!(ttf, &stm_path, 14, 30)
        })
    }
}

fn load_some_textures(
    text_man: &mut TextureManager,
    fonts: &FontCollection,
) -> Result<(), String> {

    text_man.insert_surface(
        0,
        fonts.b612_regular[&42].font
            .render("Hello there")
            .blended(Color::RGB(0, 0, 0xaf))
            .map_err(|e| e.to_string())?,
    );

    text_man.insert_surface(
        1,
        fonts.vt323_regular[&30].font
            .render("What's up!")
            .blended(Color::RGB(0, 0, 0xaf))
            .map_err(|e| e.to_string())?,
    );

    text_man.insert_surface(
        2,
        fonts.share_tech_mono_regular[&14].font
            .render("SUP!")
            .blended(Color::RGB(0, 0, 0xaf))
            .map_err(|e| e.to_string())?,
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_cxt = sdl2::init()?;
    let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let video_sys = sdl_cxt.video()?;

    let win = video_sys
        .window("RUST TTF", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = win
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_cxt.event_pump()?;

    let text_mix = canvas.texture_creator();
    let mut text_man = TextureManager::new(&text_mix);

    let font_collection = FontCollection::new(&ttf)?;

    load_some_textures(&mut text_man, &font_collection)?;

    let mut angle = 0.;
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        if let Some(text) = text_man.get_texture(0) {
            let TextureQuery { width, height, .. } = text.get_texture_info_ref();
            canvas.copy(
                text.get_texture_ref(),
                None,
                Rect::new(23, 404, *width, *height),
            )?;
        }

        if let Some(text) = text_man.get_texture(1) {
            let TextureQuery { width, height, .. } = text.get_texture_info_ref();
            canvas.copy_ex(
                text.get_texture_ref(),
                None,
                Rect::new(323, 104, *width, *height),
                angle,
                None,
                false,
                false
            )?;
        }

        if let Some(text) = text_man.get_texture(2) {
            let TextureQuery { width, height, .. } = text.get_texture_info_ref();
            let center = canvas.viewport().center();

            let x = center.x() - (width / 2) as i32;
            let y = center.y() - (height / 2) as i32;

            canvas.copy_ex(
                text.get_texture_ref(),
                None,
                Rect::new(x + 80, y + 144, *width , *height ),
                angle,
                None,
                false,
                false
            )?;
        }

        canvas.present();
        angle += 0.02;
    }

    Ok(())
}
