#include <iostream>
#include <string>
#include <vector>

#include <range/v3/view/filter.hpp>
#include <range/v3/view/transform.hpp>
using std::cout;

struct container {
    std::string value;
};

std::ostream& operator<<(std::ostream& os, const container& c)
{
    os << c.value;
    return os;
}

int main()
{
    std::vector<int> const vi{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    using namespace ranges;
    auto rng = vi | views::filter([](int i) { return i % 2 == 0; }) |
        views::transform([](int i) { return container{ std::to_string(i)}; });
    // prints: [2,4,6,8,10]
    cout << rng << '\n';
    for (auto c : rng) {
        cout << typeid(c).name() << std::endl;
    }
}
