use glamour::{Box3, Point2, Point3, Size2, Size3, Transform3, Unit, Vector2, Vector3};

// Model Space

pub struct ModelSpace;
impl Unit for ModelSpace {
    type Scalar = f32;
}

pub type ModelVector = Vector3<ModelSpace>;
pub type ModelPoint = Point3<ModelSpace>;
pub type ModelScalar = <ModelSpace as Unit>::Scalar;
pub type ModelBox = Box3<ModelSpace>;

// Model -> World Transformation

pub type ModelToWorldTransform = Transform3<ModelSpace, WorldSpace>;

// World Space

pub struct WorldSpace;
impl Unit for WorldSpace {
    type Scalar = f32;
}

pub type WorldVector = Vector3<WorldSpace>;
pub type WorldPoint = Point3<WorldSpace>;
pub type WorldScalar = <WorldSpace as Unit>::Scalar;

pub type WorldSize = Size3<WorldSpace>;

pub type WorldBox = Box3<WorldSpace>;

// World -> View Transformation

pub type WorldToViewTransform = Transform3<WorldSpace, ViewSpace>;

// View Space

pub struct ViewSpace;
impl Unit for ViewSpace {
    type Scalar = f32;
}

pub type ViewVector = Vector3<ViewSpace>;
pub type ViewPoint = Point3<ViewSpace>;
pub type ViewScalar = <ViewSpace as Unit>::Scalar;

// View -> Clip Transformation

pub type ViewToClipTransform = Transform3<ViewSpace, ClipSpace>;

// Clip Space

pub struct ClipSpace;
impl Unit for ClipSpace {
    type Scalar = f32;
}

pub type ClipVector = Vector3<ClipSpace>;
pub type ClipPoint = Point3<ClipSpace>;
pub type ClipScalar = <ClipSpace as Unit>::Scalar;

// Clip -> Screen Transform

pub type ClipToScreenTransform = Transform3<ClipSpace, ScreenSpace>;

// Screen Space

pub struct ScreenSpace;
impl Unit for ScreenSpace {
    type Scalar = f32;
}

pub type ScreenVector = Vector2<ScreenSpace>;
pub type ScreenPoint = Point2<ScreenSpace>;
pub type ScreenScalar = <ScreenSpace as Unit>::Scalar;
