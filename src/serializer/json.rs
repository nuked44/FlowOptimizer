use super::{
    ResultItem, ResultMachine, ResultRecipe, SerializableItem, SerializableMachine,
    SerializableRecipe, Serializer,
};

pub struct Json;

impl Serializer for Json {
    fn serialize(
        &self,
        path: &str,
        items: Option<Vec<SerializableItem>>,
        machines: Option<Vec<Option<SerializableMachine>>>,
        recipes: Option<Vec<SerializableRecipe>>,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn deserialize<'a>(
        &self,
        path: String,
    ) -> Result<(ResultItem<'a>, ResultMachine, ResultRecipe<'a>), std::io::Error> {
        todo!()
    }
}
