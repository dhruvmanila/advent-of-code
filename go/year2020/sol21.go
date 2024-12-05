package year2020

import (
	"fmt"
	"sort"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type food struct {
	ingredients set.Set[string]
	allergens   set.Set[string]
}

func identifyAllergens(foods []*food) (map[string]string, int) {
	candidates := make(map[string]set.Set[string])
	ingredientCount := counter.New[string]()
	for _, food := range foods {
		ingredientCount.Update(counter.NewFromSlice(food.ingredients.ToSlice()))
		food.allergens.ForEach(func(allergen string) {
			if candidates[allergen] == nil {
				candidates[allergen] = food.ingredients
			}
			candidates[allergen] = candidates[allergen].Intersection(food.ingredients)
		})
	}

	// allergens is a map from an allergen to its respective ingredient.
	allergens := make(map[string]string, len(candidates))
	for len(candidates) != 0 {
		for allergen, ingredients := range candidates {
			if ingredients.Len() == 1 {
				ingredient := ingredients.Pop()
				allergens[allergen] = ingredient
				ingredientCount.Delete(ingredient)
				delete(candidates, allergen)
				for _, otherIngredients := range candidates {
					otherIngredients.Remove(ingredient)
				}
			}
		}
	}
	return allergens, ingredientCount.Total()
}

func parseFoods(lines []string) []*food {
	foods := make([]*food, 0, len(lines))
	for _, line := range lines {
		data := strings.Split(line, " (contains ")
		ingredients := set.New[string]()
		for _, ingredient := range strings.Fields(data[0]) {
			ingredients.Add(ingredient)
		}
		allergens := set.New[string]()
		for _, allergen := range strings.Split(strings.TrimSuffix(data[1], ")"), ", ") {
			allergens.Add(allergen)
		}
		foods = append(foods, &food{
			ingredients: ingredients,
			allergens:   allergens,
		})
	}
	return foods
}

func Sol21(input string) (string, error) {
	lines := util.ReadLines(input)

	allergens, count := identifyAllergens(parseFoods(lines))

	keys := make([]string, 0, len(allergens))
	for allergen := range allergens {
		keys = append(keys, allergen)
	}
	sort.Strings(keys)
	ingredients := make([]string, len(keys))
	for i, key := range keys {
		ingredients[i] = allergens[key]
	}

	return fmt.Sprintf("21.1: %d\n21.2: %s\n", count, strings.Join(ingredients, ",")), nil
}
