//! Slider style

use iced::widget::slider::Appearance;
use iced_native::widget::slider::Handle;
use iced_native::widget::vertical_slider::HandleShape;

use crate::get_colors;
use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Slider {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Slider::Custom(Box::new(tuple))
    }
}

impl iced::widget::slider::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            rail_colors: (colors.secondary, colors.primary),
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.primary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            rail_colors: (colors.secondary, colors.secondary),
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
        }
    }

    fn dragging(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            rail_colors: (colors.secondary, colors.secondary),
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
        }
    }
}
