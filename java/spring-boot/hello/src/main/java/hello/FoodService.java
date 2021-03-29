package hello;

import java.util.List;
import java.util.Optional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class FoodService {

    @Autowired
    private FoodRepository repository;

    public List<Food> findAll() {
        var assets = (List<Food>) repository.findAll();
        return assets;
    }

    @Transactional(readOnly = true)
    public Optional<Food> findByBarcode(String barcode) {
        return repository.findByBarcode(barcode);
    }

    @Transactional
    public Food createFood(String name, String barcode, Optional<Integer> calories) {
        Food food = Food.builder().name(name).barcode(barcode).build();
        if (calories.isPresent()) {
            food.setCalories(calories.get());
        }
        repository.save(food);
        return food;
    }
}
