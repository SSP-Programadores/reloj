use std::fmt;

struct NumberDisplay {
    limit: u8,
    value: u8,
}

impl NumberDisplay {
    fn new(limit: u8) -> NumberDisplay {
        NumberDisplay {
            limit,
            value: 0,
        }
    }

    fn reinciar(&mut self) {
        self.value = 0;
    }

    fn incrementar(&mut self) {
        if self.value < self.limit {
            self.value += 1;
        }
        else {
            self.reinciar();
        }
        
    }
}

pub struct ClockDisplay {
    hours: NumberDisplay,
    minutes: NumberDisplay,
    segundos: NumberDisplay,
}

impl ClockDisplay {
    pub fn new() -> ClockDisplay {
        ClockDisplay {
            hours: NumberDisplay::new(23),
            minutes: NumberDisplay::new(59),
            segundos: NumberDisplay::new(59),
        }
    }

    pub fn funciona(&mut self) {
        self.segundos.incrementar();
        if self.segundos.value == 0 {
            self.minutes.incrementar();
        } 
    }

    fn normalizar(n: u8) -> String {
        if n < 10 {
            let mut cadena = "0".to_string();
            let digito = &n.to_string();
            cadena.push_str(digito);
            return cadena;
        }
        else {
            return n.to_string();
        }
    }
}

impl fmt::Display for ClockDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let horas = ClockDisplay::normalizar(self.hours.value);
        let minutos = ClockDisplay::normalizar(self.minutes.value);
        let segundos = ClockDisplay::normalizar(self.segundos.value);        
        
        write!(f, "({}:{}:{})", horas, minutos, segundos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
