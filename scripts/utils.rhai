fn expand_actions(ignore) {
    let orig = this.actions;
    let new_structure = #{};
    for key in orig.keys() {
        let value = orig[key];
        let description = value["descr"];
        let params = value["params"];
        let properties = #{};

        for param_key in params.keys() {
            let param_value = params[param_key];
            properties[param_key] = #{
                type: param_value["type"],
                description: param_value["descr"]
            };
        };

        new_structure[key] = #{
            type: "object",
            description: description,
            properties: properties
        };
    }
    this.actions = new_structure;
};

fn get_relevant(ignore) {
    let obj = #{};
    for field in this.relevant {
      obj[field] = this[field];
    };
    obj 
};

fn show_fields(from, fields) {
    let obj = #{};
    for field in fields {
      obj[field] = from[field];
    };
    obj.to_json()     
}

