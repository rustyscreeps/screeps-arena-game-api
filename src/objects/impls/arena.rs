#[cfg(feature = "enable-body-part")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-body-part")))]
mod body_part;

#[cfg(feature = "enable-body-part")]
pub use self::body_part::{BodyPart, BODY_PART_PROTOTYPE};

#[cfg(feature = "enable-bonus-flag")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-bonus-flag")))]
mod bonus_flag;

#[cfg(feature = "enable-bonus-flag")]
pub use self::bonus_flag::{BonusFlag, BONUS_FLAG_PROTOTYPE};

#[cfg(feature = "enable-flag")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-flag")))]
mod flag;

#[cfg(feature = "enable-flag")]
pub use self::flag::{Flag, FLAG_PROTOTYPE};

#[cfg(feature = "enable-area-effect")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-area-effect")))]
mod area_effect;

#[cfg(feature = "enable-area-effect")]
pub use self::area_effect::{AreaEffect, AREA_EFFECT_PROTOTYPE};

#[cfg(feature = "enable-score")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
mod score_collector;

#[cfg(feature = "enable-score")]
pub use self::score_collector::{ScoreCollector, SCORE_COLLECTOR_PROTOTYPE};
