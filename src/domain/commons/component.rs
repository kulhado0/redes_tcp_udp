use uuid::Uuid;

pub trait Component {
    fn id(&self) -> &Uuid;

    fn name(&self) -> &str;
}