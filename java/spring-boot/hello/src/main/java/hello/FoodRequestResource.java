package hello;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

//import javax.validation.constraints.Min;
//import javax.validation.constraints.NotBlank;

@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
public class FoodRequestResource {
    private String name;
    private String barcode;
    private int calories;
}
