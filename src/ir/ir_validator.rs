use std::collections::{HashMap, HashSet};

use super::ir_type_data::{CompileError, Enum, Function, Generic, Impl, Struct, Trait, TraitType, Type, IR};

trait ValidateCollect {
    fn collect_ce(self) -> Result<(), CompileError>;
}

impl<T> ValidateCollect for T
where
    T: Iterator<Item = Result<(), CompileError>>,
{
    fn collect_ce(self) -> Result<(), CompileError> {
        self.collect::<Result<Vec<()>, CompileError>>()
            .and_then(|_| Ok(()))
    }
}

pub struct IRValidator {
    current_generics:Vec<Vec<HashMap<String,Generic>>>,
    valid_structs:HashSet<String>,
    valid_functions:Vec<HashSet<String>>,
    valid_enums:HashSet<String>,
    valid_traits:HashSet<String>,
    valid_impls:HashSet<usize>,
}


impl IRValidator {
    pub fn validate(ir:&IR) -> Result<(), CompileError> {
        let mut validator = Self{ current_generics: vec![vec![]], valid_structs: [].into(), valid_functions: [[].into()].into(), valid_enums: [].into(), valid_traits: [].into(), valid_impls: [].into() };
        ir.structs
            .iter()
            .map(|struct_| {
                    if !validator.valid_structs.contains(struct_.0){
                        let res = validator.validate_struct(&ir, struct_.1);
                        if res.is_ok() {
                            validator.valid_structs.insert(struct_.0.clone());
                        }
                        validator.valid_structs.insert(struct_.0.clone());
                        res
                    }else{
                        Ok(())
                    }
                }
            )
            .collect_ce()?;
        Ok(())
    }
    fn new_type_generics_to_check<T:FnOnce()->Result<(),CompileError>>(&mut self,type_validator:T)->Result<(),CompileError>{
        self.current_generics.push(vec![]);
        type_validator()?;
        self.current_generics.pop();
        Ok(())
    }
    fn add_generic(&mut self, generic: &Generic){
        self.current_generics.last_mut().unwrap().last_mut().unwrap().insert(generic.name.clone(), generic.clone());
    }
    fn validate_struct(&mut self, ir:&IR, struct_: &Struct) -> Result<(), CompileError> {
        self.new_type_generics_to_check(||{
            struct_
                .generics
                .iter()
                .map(|generic| self.validate_generic(&ir,generic))
                .collect_ce()?;
            Ok(())
        })
    }
    fn validate_generic(&mut self, ir:&IR, generic: &Generic) -> Result<(), CompileError> {
        generic.constraits.iter().map(|constraint|{self.validate_trait_type(&ir,constraint)}).collect_ce()?;
        self.current_generics.last_mut().unwrap().insert(generic.name.clone(), generic.clone());
        Ok(())
    }
    fn validate_trait_type(&mut self, ir:&IR, trait_:&TraitType)-> Result<(),CompileError>{
        let actual_trait = ir.traits.get(&trait_.name).ok_or(CompileError::NoTraitWithThatNameFound(trait_.name.clone()))?;
        if actual_trait.generics.len() != trait_.types_in_generics.len(){
            Err(CompileError::NumberOfGenericsAndTypesGivenDoNotMatchInItem(trait_.name.clone(), actual_trait.generics.len(), trait_.types_in_generics.len()))?;
        }
        if !self.valid_traits.contains(&trait_.name){
            self.validate_trait(&ir,&actual_trait)?;
        }
        //actual_trait.generics.iter().zip(trait_.types_in_generics.iter()).map(|(generic,type_given)|{self.validate_type_in_constraints(&ir,generic.con)}).collect_ce()?;
        Ok(())
    }
    fn validate_trait(&mut self, ir:&IR, trait_:&Trait)->Result<(),CompileError>{
        self.current_generics.push(HashMap::new());
        trait_
            .generics
            .iter()
            .map(|generic| self.validate_generic(&ir,generic))
            .collect_ce()?;
        validatetrait_.func_tag
        self.current_generics.pop();
        Ok(())
    }
}