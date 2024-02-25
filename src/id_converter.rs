use crate::utils;

pub struct IDConverter {
    alphabets: String,
    numbers: String
}

impl IDConverter {
    fn convert_base(&self, input: String, translator: &String, convert_translator: &String, shift_left: bool) -> String {
        let input = input.as_str();
        let translator = translator.as_str();
        let convert_translator = convert_translator.as_str();

        let mut base_x: usize = 0;
        let base_value: usize = translator.chars().count();

        for char in input.chars() {
            let mut char_index = translator.chars().position(|c| c == char).unwrap_or(0) + 1;
            char_index -= if shift_left { 1 } else { 0 };

            base_x = base_x * base_value + char_index;
        }

        if base_x != 0 {
            let mut result = String::new();
            let new_base_value: usize = convert_translator.chars().count();

            while base_x > 0 {
                let mut translated_position = base_x % new_base_value;
                translated_position -= if shift_left { 1 } else { 0 };
                
                let translated_char = convert_translator.chars().nth(translated_position);
                let mut char_to_use: char = '0';
                if translated_char.is_some() {
                    char_to_use = translated_char.unwrap();
                }

                result.push(char_to_use);
                base_x /= new_base_value;
            }

            utils::reverse_string(result.as_str())
        } else {
            String::from(convert_translator.chars().nth(0).unwrap())
        }
    }

    pub fn new(alphabets: &String, numbers: &String) -> Self {
        Self { alphabets: alphabets.to_owned(), numbers: numbers.to_owned() }
    }

    pub fn to_short(&self, input: u64) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.to_string();
        let converted_to_base = self.convert_base(input, &self.numbers, &self.alphabets, true);
        Ok(utils::reverse_string(converted_to_base.as_str()))
    }

    pub fn to_number(&self, input: String) -> Result<u64, Box<dyn std::error::Error>> {
        let converted_to_base = self.convert_base(utils::reverse_string(input.as_str()), &self.alphabets, &self.numbers, false);
        let id_from_converted = converted_to_base.parse::<u64>();
        match id_from_converted {
            Ok(id) => Ok(id),
            Err(_) => Err("Transformed ID is not a number. Input possibly error/corrupted.".into())
        }
    }
}