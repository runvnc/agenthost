// Enum for different billing types
enum BillingType {
    Image { width: u64, height: u64, num_steps: u64 },
    Text { num_input_tokens: u64, num_output_tokens: u64 },
}

// Struct for service information
struct ServiceInfo {
    base_cost: f64,
    markup_percent: f64,
}

impl ServiceInfo {
    fn calculate_final_cost(&self, base_cost: f64) -> f64 {
        base_cost * (1.0 + self.markup_percent / 100.0)
    }
}

// Trait for cost calculation
trait CostCalculator {
    fn calculate_cost(&self, billing_type: &BillingType) -> f64;
}

// Base implementation for Text Generation Services
struct BaseTextGenerator {
    service_info: ServiceInfo,
}

impl BaseTextGenerator {
    fn base_text_cost(&self, billing_type: &BillingType) -> f64 {
        match billing_type {
            BillingType::Text { num_input_tokens, num_output_tokens } => {
                let base_cost = (*num_input_tokens as f64 + *num_output_tokens as f64) * self.service_info.base_cost;
                self.service_info.calculate_final_cost(base_cost)
            }
            _ => 0.0,
        }
    }
}

// Base implementation for Image Generation Services
struct BaseImageGenerator {
    service_info: ServiceInfo,
}

impl BaseImageGenerator {
    fn base_image_cost(&self, billing_type: &BillingType) -> f64 {
        match billing_type {
            BillingType::Image { width, height, num_steps } => {
                let base_cost = (*width * *height * *num_steps) as f64 * self.service_info.base_cost;
                self.service_info.calculate_final_cost(base_cost)
            }
            _ => 0.0,
        }
    }
}

// Specific service implementations
struct OpenAITextGenerator {
    base_generator: BaseTextGenerator,
}

impl CostCalculator for OpenAITextGenerator {
    fn calculate_cost(&self, billing_type: &BillingType) -> f64 {
        // Additional logic specific to OpenAI's text generator can be added here
        self.base_generator.base_text_cost(billing_type)
    }
}

struct LocalImageGenerator {
    base_generator: BaseImageGenerator,
}

impl CostCalculator for LocalImageGenerator {
    fn calculate_cost(&self, billing_type: &BillingType) -> f64 {
        // Additional logic specific to the local image generator can be added here
        self.base_generator.base_image_cost(billing_type)
    }
}

// Example usage
fn main() {
    let text_service_info = ServiceInfo { base_cost: 0.01, markup_percent: 10.0 };
    let openai_text_generator = OpenAITextGenerator {
        base_generator: BaseTextGenerator { service_info: text_service_info },
    };

    let billing_info = BillingType::Text { num_input_tokens: 100, num_output_tokens: 50 };
    let cost = openai_text_generator.calculate_cost(&billing_info);
    println!("Cost for text generation: {}", cost);
}

