use peacock::graphics::{self, Color, DrawImageParams, Image};
use peacock::{ContextBuilder, Result, State};

type Context = peacock::Context<()>;

struct ColorPaletteExample {
    swatches: Vec<Image>,
}

impl ColorPaletteExample {
    fn new(ctx: &mut Context) -> Result<Self> {
        let all_colors = vec![
            // Pinks
            Color::PINK,
            Color::LIGHT_PINK,
            Color::HOT_PINK,
            Color::DEEP_PINK,
            Color::PALE_VIOLET_RED,
            Color::MEDIUM_VIOLET_RED,
            // Reds
            Color::LIGHT_SALMON,
            Color::SALMON,
            Color::DARK_SALMON,
            Color::LIGHT_CORAL,
            Color::INDIAN_RED,
            Color::CRIMSON,
            Color::FIREBRICK,
            Color::DARK_RED,
            Color::RED,
            // Oranges
            Color::ORANGE_RED,
            Color::TOMATO,
            Color::CORAL,
            Color::DARK_ORANGE,
            Color::ORANGE,
            // Yellows
            Color::YELLOW,
            Color::LIGHT_YELLOW,
            Color::LEMON_CHIFFON,
            Color::LIGHT_GOLDENROD_YELLOW,
            Color::PAPAYA_WHIP,
            Color::MOCCASIN,
            Color::PEACH_PUFF,
            Color::PALE_GOLDENROD,
            Color::KHAKI,
            Color::DARK_KHAKI,
            Color::GOLD,
            // Browns
            Color::CORNSILK,
            Color::BLANCHED_ALMOND,
            Color::BISQUE,
            Color::NAVAJO_WHITE,
            Color::WHEAT,
            Color::BURLYWOOD,
            Color::TAN,
            Color::ROSY_BROWN,
            Color::SANDY_BROWN,
            Color::GOLDENROD,
            Color::DARK_GOLDENROD,
            Color::PERU,
            Color::CHOCOLATE,
            Color::SADDLE_BROWN,
            Color::SIENNA,
            Color::BROWN,
            Color::MAROON,
            // Greens
            Color::DARK_OLIVE_GREEN,
            Color::OLIVE,
            Color::OLIVE_DRAB,
            Color::YELLOW_GREEN,
            Color::LIME_GREEN,
            Color::LIME,
            Color::LAWN_GREEN,
            Color::CHARTREUSE,
            Color::GREEN_YELLOW,
            Color::SPRING_GREEN,
            Color::MEDIUM_SPRING_GREEN,
            Color::LIGHT_GREEN,
            Color::PALE_GREEN,
            Color::DARK_SEA_GREEN,
            Color::MEDIUM_AQUAMARINE,
            Color::MEDIUM_SEA_GREEN,
            Color::SEA_GREEN,
            Color::FOREST_GREEN,
            Color::GREEN,
            Color::DARK_GREEN,
            // Cyans
            Color::AQUA,
            Color::CYAN,
            Color::LIGHT_CYAN,
            Color::PALE_TURQUOISE,
            Color::AQUAMARINE,
            Color::TURQUOISE,
            Color::MEDIUM_TURQUOISE,
            Color::DARK_TURQUOISE,
            Color::LIGHT_SEA_GREEN,
            Color::CADET_BLUE,
            Color::DARK_CYAN,
            Color::TEAL,
            // Blues
            Color::LIGHT_STEEL_BLUE,
            Color::POWDER_BLUE,
            Color::LIGHT_BLUE,
            Color::SKY_BLUE,
            Color::LIGHT_SKY_BLUE,
            Color::DEEP_SKY_BLUE,
            Color::DODGER_BLUE,
            Color::CORNFLOWER_BLUE,
            Color::STEEL_BLUE,
            Color::ROYAL_BLUE,
            Color::BLUE,
            Color::MEDIUM_BLUE,
            Color::DARK_BLUE,
            Color::NAVY,
            Color::MIDNIGHT_BLUE,
            // Purples, violets, and magentas
            Color::LAVENDER,
            Color::THISTLE,
            Color::PLUM,
            Color::VIOLET,
            Color::ORCHID,
            Color::FUCHSIA,
            Color::MAGENTA,
            Color::MEDIUM_ORCHID,
            Color::MEDIUM_PURPLE,
            Color::BLUE_VIOLET,
            Color::DARK_VIOLET,
            Color::DARK_ORCHID,
            Color::DARK_MAGENTA,
            Color::PURPLE,
            Color::INDIGO,
            Color::DARK_SLATE_BLUE,
            Color::SLATE_BLUE,
            Color::MEDIUM_SLATE_BLUE,
            // Whites
            Color::WHITE,
            Color::SNOW,
            Color::HONEYDEW,
            Color::MINT_CREAM,
            Color::AZURE,
            Color::ALICE_BLUE,
            Color::GHOST_WHITE,
            Color::WHITE_SMOKE,
            Color::SEASHELL,
            Color::BEIGE,
            Color::OLD_LACE,
            Color::FLORAL_WHITE,
            Color::IVORY,
            Color::ANTIQUE_WHITE,
            Color::LINEN,
            Color::LAVENDER_BLUSH,
            Color::MISTY_ROSE,
            // Grays and blacks
            Color::GAINSBORO,
            Color::LIGHT_GRAY,
            Color::SILVER,
            Color::DARK_GRAY,
            Color::GRAY,
            Color::DIM_GRAY,
            Color::LIGHT_SLATE_GRAY,
            Color::SLATE_GRAY,
            Color::DARK_SLATE_GRAY,
            Color::BLACK,
        ];

        let mut swatches = Vec::with_capacity(all_colors.len());
        for color in all_colors {
            swatches.push(Image::from_color(ctx, (32, 32).into(), color)?);
        }

        Ok(Self { swatches })
    }
}

impl State for ColorPaletteExample {
    type Context = ();

    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        graphics::clear(ctx, Color::BLACK);

        let (width, height) = (12, 12);

        for x in 0..width {
            for y in 0..height {
                let index = x + (y * width);
                if index > self.swatches.len() - 1 {
                    break;
                }

                graphics::draw(
                    ctx,
                    &self.swatches[index],
                    &DrawImageParams {
                        position: (x as f32 * 32.0, y as f32 * 32.0).into(),
                        ..Default::default()
                    },
                )?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Color Palette", 384, 384)
        .build_empty()?
        .run_with_result(ColorPaletteExample::new)
}
