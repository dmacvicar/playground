package hello;

import java.util.List;
import java.util.Optional;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;

@RestController
public class HelloController {

    @Autowired
    private FoodService foodService;
    
    @RequestMapping("/")
    public String index() {
        return "Greetings from Spring Boot!";
    }

    @GetMapping(path = "/foods")
    public ResponseEntity<List<Food>> findAssets() {
        var foods = foodService.findAll();
        return ResponseEntity.accepted().body(foods);
    }

    @GetMapping(path = "/food/{barcode}")
    public ResponseEntity<Food> findFood(@PathVariable String barcode) {
        Optional<Food> foodOpt = foodService.findByBarcode(barcode);
        return ResponseEntity.of(foodOpt);
    }

    @PostMapping(path = "/foods")
    public void createFood(@RequestBody FoodRequestResource resource) {
        foodService.createFood(resource.getName(), resource.getBarcode(), Optional.of(resource.getCalories()));
     }
}
