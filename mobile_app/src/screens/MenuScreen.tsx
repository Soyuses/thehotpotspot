import React, { useState, useEffect } from 'react';
import {
  View,
  StyleSheet,
  ScrollView,
  RefreshControl,
  Alert,
  Image,
} from 'react-native';
import {
  Text,
  Card,
  Title,
  Paragraph,
  Button,
  Chip,
  ActivityIndicator,
  Searchbar,
} from 'react-native-paper';
import { menuAPI } from '../services/api';
import { useTheme } from '../contexts/ThemeContext';

interface Dish {
  id: string;
  name: string;
  description: string;
  price: number;
  image: string;
  category: string;
  ingredients: string[];
  allergens: string[];
  preparationTime: number;
  rating: number;
  isAvailable: boolean;
}

const MenuScreen: React.FC = () => {
  const [dishes, setDishes] = useState<Dish[]>([]);
  const [filteredDishes, setFilteredDishes] = useState<Dish[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const { theme } = useTheme();

  const categories = [
    { id: 'all', name: 'Все' },
    { id: 'hot', name: 'Горячие блюда' },
    { id: 'cold', name: 'Холодные закуски' },
    { id: 'drinks', name: 'Напитки' },
    { id: 'desserts', name: 'Десерты' },
  ];

  useEffect(() => {
    loadMenu();
  }, []);

  useEffect(() => {
    filterDishes();
  }, [dishes, searchQuery, selectedCategory]);

  const loadMenu = async () => {
    try {
      setIsLoading(true);
      const response = await menuAPI.getMenu();
      
      if (response.success && response.data) {
        setDishes(response.data);
      } else {
        // Mock data for demo
        setDishes(getMockDishes());
      }
    } catch (error) {
      console.error('Error loading menu:', error);
      setDishes(getMockDishes());
    } finally {
      setIsLoading(false);
    }
  };

  const getMockDishes = (): Dish[] => [
    {
      id: '1',
      name: 'Плов с бараниной',
      description: 'Традиционный узбекский плов с нежной бараниной, морковью и рисом',
      price: 25.00,
      image: 'https://via.placeholder.com/300x200?text=Плов',
      category: 'hot',
      ingredients: ['Рис', 'Баранина', 'Морковь', 'Лук', 'Специи'],
      allergens: ['Глютен'],
      preparationTime: 45,
      rating: 4.8,
      isAvailable: true,
    },
    {
      id: '2',
      name: 'Хинкали с мясом',
      description: 'Грузинские хинкали с сочной мясной начинкой',
      price: 18.00,
      image: 'https://via.placeholder.com/300x200?text=Хинкали',
      category: 'hot',
      ingredients: ['Мука', 'Говядина', 'Свинина', 'Лук', 'Зелень'],
      allergens: ['Глютен'],
      preparationTime: 30,
      rating: 4.9,
      isAvailable: true,
    },
    {
      id: '3',
      name: 'Хачапури по-аджарски',
      description: 'Лодочка из теста с сыром и яйцом',
      price: 22.00,
      image: 'https://via.placeholder.com/300x200?text=Хачапури',
      category: 'hot',
      ingredients: ['Мука', 'Сыр', 'Яйца', 'Масло'],
      allergens: ['Глютен', 'Молочные продукты', 'Яйца'],
      preparationTime: 25,
      rating: 4.7,
      isAvailable: true,
    },
    {
      id: '4',
      name: 'Салат "Оливье"',
      description: 'Классический салат с колбасой, картофелем и майонезом',
      price: 12.00,
      image: 'https://via.placeholder.com/300x200?text=Оливье',
      category: 'cold',
      ingredients: ['Картофель', 'Колбаса', 'Яйца', 'Огурцы', 'Майонез'],
      allergens: ['Яйца', 'Молочные продукты'],
      preparationTime: 15,
      rating: 4.5,
      isAvailable: true,
    },
    {
      id: '5',
      name: 'Чай с лимоном',
      description: 'Ароматный черный чай с долькой лимона',
      price: 5.00,
      image: 'https://via.placeholder.com/300x200?text=Чай',
      category: 'drinks',
      ingredients: ['Чай', 'Лимон', 'Сахар'],
      allergens: [],
      preparationTime: 5,
      rating: 4.3,
      isAvailable: true,
    },
  ];

  const filterDishes = () => {
    let filtered = dishes;

    // Filter by category
    if (selectedCategory !== 'all') {
      filtered = filtered.filter(dish => dish.category === selectedCategory);
    }

    // Filter by search query
    if (searchQuery.trim()) {
      filtered = filtered.filter(dish =>
        dish.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        dish.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        dish.ingredients.some(ingredient =>
          ingredient.toLowerCase().includes(searchQuery.toLowerCase())
        )
      );
    }

    setFilteredDishes(filtered);
  };

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadMenu();
    setRefreshing(false);
  };

  const handleOrderDish = async (dish: Dish) => {
    if (!dish.isAvailable) {
      Alert.alert('Недоступно', 'Это блюдо временно недоступно');
      return;
    }

    Alert.alert(
      'Заказ',
      `Добавить "${dish.name}" в корзину за ${dish.price} GEL?`,
      [
        { text: 'Отмена', style: 'cancel' },
        {
          text: 'Заказать',
          onPress: async () => {
            try {
              const response = await menuAPI.placeOrder({
                dishId: dish.id,
                quantity: 1,
                totalPrice: dish.price,
              });

              if (response.success) {
                Alert.alert(
                  'Заказ принят!',
                  'Ваш заказ готовится. Покажите QR-код на кассе для получения чека.',
                  [
                    {
                      text: 'OK',
                      onPress: () => {
                        // Navigate to order details or QR code
                        console.log('Order placed:', response.data);
                      },
                    },
                  ]
                );
              } else {
                Alert.alert('Ошибка', 'Не удалось оформить заказ');
              }
            } catch (error) {
              Alert.alert('Ошибка', 'Произошла ошибка при оформлении заказа');
            }
          },
        },
      ]
    );
  };

  const formatPrice = (price: number) => {
    return `${price.toFixed(2)} GEL`;
  };

  const renderDishCard = (dish: Dish) => (
    <Card key={dish.id} style={[styles.dishCard, { backgroundColor: theme.colors.surface }]}>
      <Card.Content style={styles.dishContent}>
        <View style={styles.dishHeader}>
          <View style={styles.dishInfo}>
            <Title style={[styles.dishName, { color: theme.colors.onSurface }]}>
              {dish.name}
            </Title>
            <Paragraph style={[styles.dishDescription, { color: theme.colors.onSurface }]}>
              {dish.description}
            </Paragraph>
          </View>
          <View style={styles.dishPrice}>
            <Text style={[styles.priceText, { color: theme.colors.primary }]}>
              {formatPrice(dish.price)}
            </Text>
          </View>
        </View>

        <View style={styles.dishDetails}>
          <View style={styles.dishMeta}>
            <Chip
              icon="clock"
              style={[styles.metaChip, { backgroundColor: theme.colors.surfaceVariant }]}
              textStyle={{ color: theme.colors.onSurface }}
            >
              {dish.preparationTime} мин
            </Chip>
            <Chip
              icon="star"
              style={[styles.metaChip, { backgroundColor: theme.colors.surfaceVariant }]}
              textStyle={{ color: theme.colors.onSurface }}
            >
              {dish.rating}
            </Chip>
          </View>

          <View style={styles.ingredientsContainer}>
            <Text style={[styles.ingredientsLabel, { color: theme.colors.onSurface }]}>
              Ингредиенты:
            </Text>
            <Text style={[styles.ingredientsText, { color: theme.colors.onSurface }]}>
              {dish.ingredients.join(', ')}
            </Text>
          </View>

          {dish.allergens.length > 0 && (
            <View style={styles.allergensContainer}>
              <Text style={[styles.allergensLabel, { color: theme.colors.error }]}>
                Аллергены: {dish.allergens.join(', ')}
              </Text>
            </View>
          )}
        </View>

        <Button
          mode="contained"
          onPress={() => handleOrderDish(dish)}
          style={[styles.orderButton, { backgroundColor: theme.colors.primary }]}
          disabled={!dish.isAvailable}
        >
          {dish.isAvailable ? 'Заказать' : 'Недоступно'}
        </Button>
      </Card.Content>
    </Card>
  );

  if (isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={theme.colors.primary} />
        <Text style={[styles.loadingText, { color: theme.colors.onSurface }]}>
          Загрузка меню...
        </Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={handleRefresh} />
        }
      >
        {/* Search Bar */}
        <View style={styles.searchContainer}>
          <Searchbar
            placeholder="Поиск блюд..."
            onChangeText={setSearchQuery}
            value={searchQuery}
            style={styles.searchBar}
          />
        </View>

        {/* Categories */}
        <View style={styles.categoriesContainer}>
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {categories.map((category) => (
              <Chip
                key={category.id}
                selected={selectedCategory === category.id}
                onPress={() => setSelectedCategory(category.id)}
                style={[
                  styles.categoryChip,
                  selectedCategory === category.id && { backgroundColor: theme.colors.primary },
                ]}
                textStyle={{
                  color: selectedCategory === category.id ? theme.colors.onPrimary : theme.colors.onSurface,
                }}
              >
                {category.name}
              </Chip>
            ))}
          </ScrollView>
        </View>

        {/* Dishes List */}
        <View style={styles.dishesContainer}>
          {filteredDishes.length === 0 ? (
            <View style={styles.emptyContainer}>
              <Text style={[styles.emptyText, { color: theme.colors.onSurface }]}>
                Блюда не найдены
              </Text>
              <Paragraph style={[styles.emptySubtext, { color: theme.colors.onSurface }]}>
                Попробуйте изменить поисковый запрос или выберите другую категорию
              </Paragraph>
            </View>
          ) : (
            filteredDishes.map(renderDishCard)
          )}
        </View>
      </ScrollView>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5f5f5',
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
  },
  scrollView: {
    flex: 1,
  },
  searchContainer: {
    padding: 16,
    paddingBottom: 8,
  },
  searchBar: {
    elevation: 2,
    borderRadius: 8,
  },
  categoriesContainer: {
    paddingHorizontal: 16,
    paddingBottom: 16,
  },
  categoryChip: {
    marginRight: 8,
    borderRadius: 20,
  },
  dishesContainer: {
    padding: 16,
    paddingTop: 0,
  },
  dishCard: {
    marginBottom: 16,
    elevation: 2,
    borderRadius: 12,
  },
  dishContent: {
    padding: 16,
  },
  dishHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 12,
  },
  dishInfo: {
    flex: 1,
    marginRight: 12,
  },
  dishName: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  dishDescription: {
    fontSize: 14,
    lineHeight: 20,
  },
  dishPrice: {
    alignItems: 'flex-end',
  },
  priceText: {
    fontSize: 20,
    fontWeight: 'bold',
  },
  dishDetails: {
    marginBottom: 16,
  },
  dishMeta: {
    flexDirection: 'row',
    marginBottom: 12,
    gap: 8,
  },
  metaChip: {
    borderRadius: 16,
  },
  ingredientsContainer: {
    marginBottom: 8,
  },
  ingredientsLabel: {
    fontSize: 14,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  ingredientsText: {
    fontSize: 13,
    lineHeight: 18,
  },
  allergensContainer: {
    marginBottom: 8,
  },
  allergensLabel: {
    fontSize: 12,
    fontWeight: 'bold',
  },
  orderButton: {
    borderRadius: 8,
  },
  emptyContainer: {
    alignItems: 'center',
    padding: 40,
  },
  emptyText: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  emptySubtext: {
    fontSize: 14,
    textAlign: 'center',
    lineHeight: 20,
  },
});

export default MenuScreen;

