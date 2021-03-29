package hello;

import java.util.Optional;

import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;
import org.springframework.transaction.annotation.Transactional;

@Repository
public interface FoodRepository extends CrudRepository<Food, Long> {

   @Transactional(readOnly = true)
   public Optional<Food> findByBarcode(String barcode);
    
}
