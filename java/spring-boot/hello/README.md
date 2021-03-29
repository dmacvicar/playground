
# Spring Boot API example

A simple and dumb CRUD for foods, barcodes and calories, to learn Spring-Boot.

./build/libs/hello-0.0.1-SNAPSHOT.jar

## Notes

### Boilerplate

```
compileOnly 'org.projectlombok:lombok'
annotationProcessor 'org.projectlombok:lombok'
```

Saves writing boilerplate, like setters (@Data) and builders (@Builder).

### Metrics and Prometheus

For general metrics, add  `org.springframework.boot:spring-boot-starter-actuator`. For exposing Prometheus metrics, aadd `io.micrometer:micrometer-registry-prometheus` (on top of actuator). Not tested yet.

### Design

How to structure the application:

* Application
* EntityController (Use RequestEntity as a helper for returning values)
* EntityService
* Entity
* EntityRepository (DAO, extending `CrudRepository<T, IdType>`)
* Entity(Request/Reponse)Resource (constructed from Entity). This is optional, but a light DTO to handle the JSON request.

# PostgreSQL

Add to `application.properties` in the resources:

```
spring.datasource.platform=postgres
spring.datasource.url=jdbc:postgresql://localhost:5432/postgres
spring.datasource.username=postgres
spring.datasource.password=postgres
```

Add the dependency `implementation 'org.postgresql:postgresql`.
  
### Migrations

Add `org.flywaydb:flyway-core` and then add files in `src/main/resources/db/migration` in the form `V1__description.sql` (note the double underscore).

### Validations

Could not get `@Valid` and `@NonBlank` working. Could not resolve the symbols even with `org.springframework.boot:spring-boot-starter-validation` and `javax.validation:validation-api` manually added.

### Test

`@SpringBootTest` and `@AutoConfigureMockMvc`. Add a `@Autowired` `MockMvc` and you can do things like:

```java
mvc.perform(MockMvcRequestBuilders.get("/")
            .accept(MediaType.APPLICATION_JSON))
            .andExpect(status().isOk())
            .andExpect(content().string(equalTo("some response")));
```

