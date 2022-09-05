use obj::ObjData;

pub struct Object {
    object: ObjData,
}

impl Object {
    pub fn new() -> Object {
        Object {
            object: ObjData::default(),
        }
    }

    pub fn stack_layer(&mut self, coords: Vec<[f32; 3]>) {
        for coord in coords.iter() {
            self.object.position.push(coord.to_owned());
        }
    }

    pub fn export(&self, output: String) {
        self.object.save(output).unwrap();
    }
}
