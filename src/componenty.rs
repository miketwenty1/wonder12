use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct UiNode;

#[derive(Component)]
pub struct UiOverlayingExplorerButton;

#[derive(Component)]
pub struct GoToBtn;

#[derive(Component)]
pub struct DrawBtn;

#[derive(Component)]
pub struct DrawBtnImage;

#[derive(Component)]
pub struct SelectedTileUi;

#[derive(Component)]
pub struct Selected(pub Color);

#[derive(Component)]
pub struct ManualSelected;

#[derive(Component)]
pub struct DrawSelected;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub ulam: u32,
    pub quad: ulam::Quad,
    pub selected: bool,
}

#[derive(Component)]
pub struct ClearSelectionButton;

#[derive(Component)]
pub struct BuySelectionButton;

#[derive(Component)]
pub struct ZoomOutButton;

#[derive(Component)]
pub struct ZoomInButton;

#[derive(Component, Clone, Copy)]
pub struct Land;

#[derive(Component, Clone, Copy)]
pub enum BuildingStructure {
    Camp,
    Hut,
    Shack,
    House,
    Castle,
    Road,
    FirePit,
    Waterwell,
}

#[derive(Component)]
pub struct UiSideNode;

#[derive(Component)]
pub struct ToggleParent;

#[derive(Component)]
pub struct ToggleButton;

#[derive(Component)]
pub struct HideBuilding;

#[derive(Component)]
pub struct ShowColors;

#[derive(Component)]
pub struct ShowValues;

#[derive(Component)]
pub struct HideText;

#[derive(Component)]
pub struct HideTextText;

#[derive(Component)]
pub struct Toggle1Btn;

#[derive(Component)]
pub struct Toggle2Btn;

#[derive(Component)]
pub struct Toggle3Btn;

#[derive(Component)]
pub struct Toggle4Btn;

#[derive(Component)]
pub struct Toggle1BtnText;

#[derive(Component)]
pub struct Toggle2BtnText;

#[derive(Component)]
pub struct Toggle3BtnText;

#[derive(Component)]
pub struct Toggle4BtnText;

#[derive(Component)]
pub struct TileText;

#[derive(Component)]
pub struct CartButton(pub i32);

#[derive(Component)]
pub struct BlockHeightCartText;

#[derive(Component)]
pub struct CurrentBlockMessageNode;

#[derive(Component)]
pub struct BlockUiMessageItem;

#[derive(Component)]
pub struct HideMessageBtn;

#[derive(Component)]
pub struct NewBlockLnAddressButton;

#[derive(Component)]
pub struct NewBlockLnAddressText;

#[derive(Component)]
pub struct NewBlockColorButton;

#[derive(Component)]
pub struct NewBlockColorText;

#[derive(Component)]
pub struct NewBlockMessageButton;

#[derive(Component)]
pub struct NewBlockMessageText;

#[derive(Component)]
pub struct NewBlockDataButton;

#[derive(Component)]
pub struct BlockCostText;

#[derive(Component)]
pub struct AllCartConfigText;

#[derive(Component)]
pub struct AllCartConfigButton;

#[derive(Component)]
pub struct EditabledTextBox;

#[derive(Component)]
pub struct CouldBeEditabledTextBox;

#[derive(Component)]
pub struct BuyMenuButton;

#[derive(Component)]
pub struct ClipboardBtn;

#[derive(Component)]
pub struct CancelQrButton;

#[derive(Component)]
pub struct ExpirationQrText;

#[derive(Component)]
pub struct InitLoadingNode;

#[derive(Component)]
pub struct InitLoadingText;

#[derive(Component)]
pub struct BtnShowingColor;

#[derive(Component)]
pub struct UiInteractionBtn;
