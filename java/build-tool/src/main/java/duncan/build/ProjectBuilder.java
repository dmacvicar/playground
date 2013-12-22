package duncan.build;

public interface ProjectBuilder {
    
    public void java_library(String name);
    public void add_subdirectory(String name);
    
}
