import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  FlatList,
  TouchableOpacity,
  StyleSheet,
  RefreshControl,
  Alert,
  ActivityIndicator,
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import LinearGradient from 'react-native-linear-gradient';

import useAppStore from '../store';
import { MenuItem } from '../../types';

const MenuScreen: React.FC = () => {
  const [cart, setCart] = useState<{[key: string]: number}>({});
  const [showCart, setShowCart] = useState(false);

  const { menuItems, loadMenuItems, isLoading, error } = useAppStore();

  useEffect(() => {
    loadMenuItems();
  }, []);

  const handleAddToCart = (itemId: string) => {
    setCart(prev => ({
      ...prev,
      [itemId]: (prev[itemId] || 0) + 1
    }));
  };

  const handleRemoveFromCart = (itemId: string) => {
    setCart(prev => {
      const newCart = { ...prev };
      if (newCart[itemId] > 1) {
        newCart[itemId] -= 1;
      } else {
        delete newCart[itemId];
      }
      return newCart;
    });
  };

  const getTotalPrice = () => {
    return Object.entries(cart).reduce((total, [itemId, quantity]) => {
      const item = menuItems.find(i => i.id === itemId);
      return total + (item ? item.price * quantity : 0);
    }, 0);
  };

  const getTotalItems = () => {
    return Object.values(cart).reduce((total, quantity) => total + quantity, 0);
  };

  const handlePlaceOrder = () => {
    if (Object.keys(cart).length === 0) {
      Alert.alert('Корзина пуста', 'Добавьте блюда в корзину');
      return;
    }

    const orderItems = Object.entries(cart).map(([itemId, quantity]) => ({
      menuItemId: itemId,
      quantity,
    }));

    Alert.alert(
      'Подтверждение заказа',
      `Итого: $${getTotalPrice().toFixed(2)}\nКоличество позиций: ${getTotalItems()}`,
      [
        { text: 'Отмена', style: 'cancel' },
        {
          text: 'Заказать',
          onPress: () => {
            // Здесь будет логика создания заказа
            Alert.alert('Успешно!', 'Заказ создан и отправлен в ресторан');
            setCart({});
            setShowCart(false);
          }
        }
      ]
    );
  };

  const renderMenuItem = ({ item }: { item: MenuItem }) => {
    const cartQuantity = cart[item.id] || 0;
    const isAvailable = item.availability > 0 && !item.isAvailableForVoting;

    return (
      <View style={styles.menuItem}>
        <View style={styles.menuItemHeader}>
          <Text style={styles.menuItemName}>{item.name}</Text>
          <Text style={styles.menuItemPrice}>${item.price.toFixed(2)}</Text>
        </View>

        <Text style={styles.menuItemDescription}>{item.description}</Text>

        <View style={styles.menuItemDetails}>
          <View style={styles.detailItem}>
            <Icon name="schedule" size={16} color="#666" />
            <Text style={styles.detailText}>{item.cookingTimeMinutes} мин</Text>
          </View>
          <View style={styles.detailItem}>
            <Icon name="local-fire-department" size={16} color="#666" />
            <Text style={styles.detailText}>{item.totalCalories} ккал</Text>
          </View>
          <View style={styles.detailItem}>
            <Icon name="inventory" size={16} color="#666" />
            <Text style={styles.detailText}>{item.availability} шт</Text>
          </View>
        </View>

        {item.isAvailableForVoting ? (
          <View style={styles.votingBadge}>
            <Icon name="how-to-vote" size={16} color="#ff6b6b" />
            <Text style={styles.votingText}>На голосовании</Text>
          </View>
        ) : (
          <View style={styles.cartControls}>
            {cartQuantity > 0 && (
              <TouchableOpacity
                style={styles.cartButton}
                onPress={() => handleRemoveFromCart(item.id)}
              >
                <Icon name="remove" size={20} color="#fff" />
              </TouchableOpacity>
            )}
            
            {cartQuantity > 0 && (
              <Text style={styles.cartQuantity}>{cartQuantity}</Text>
            )}
            
            <TouchableOpacity
              style={[
                styles.cartButton,
                !isAvailable && styles.cartButtonDisabled
              ]}
              onPress={() => handleAddToCart(item.id)}
              disabled={!isAvailable}
            >
              <Icon name="add" size={20} color="#fff" />
            </TouchableOpacity>
          </View>
        )}
      </View>
    );
  };

  const renderCartSummary = () => {
    if (Object.keys(cart).length === 0) return null;

    return (
      <View style={styles.cartSummary}>
        <View style={styles.cartInfo}>
          <Text style={styles.cartText}>
            {getTotalItems()} позиций • ${getTotalPrice().toFixed(2)}
          </Text>
        </View>
        <TouchableOpacity
          style={styles.orderButton}
          onPress={handlePlaceOrder}
        >
          <Text style={styles.orderButtonText}>Заказать</Text>
        </TouchableOpacity>
      </View>
    );
  };

  if (isLoading && menuItems.length === 0) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#43e97b" />
        <Text style={styles.loadingText}>Загрузка меню...</Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <FlatList
        data={menuItems}
        renderItem={renderMenuItem}
        keyExtractor={(item) => item.id}
        contentContainerStyle={styles.listContainer}
        refreshControl={
          <RefreshControl
            refreshing={isLoading}
            onRefresh={loadMenuItems}
            colors={['#43e97b']}
          />
        }
        ListEmptyComponent={
          <View style={styles.emptyContainer}>
            <Icon name="restaurant-menu" size={64} color="#ccc" />
            <Text style={styles.emptyText}>Меню пусто</Text>
            <Text style={styles.emptySubtext}>
              Попробуйте обновить список
            </Text>
          </View>
        }
      />

      {renderCartSummary()}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f8f9fa',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f8f9fa',
  },
  loadingText: {
    marginTop: 10,
    fontSize: 16,
    color: '#666',
  },
  listContainer: {
    padding: 15,
  },
  menuItem: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 15,
    marginBottom: 15,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  menuItemHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  menuItemName: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    flex: 1,
  },
  menuItemPrice: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#43e97b',
  },
  menuItemDescription: {
    fontSize: 14,
    color: '#666',
    marginBottom: 12,
    lineHeight: 20,
  },
  menuItemDetails: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 15,
  },
  detailItem: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  detailText: {
    fontSize: 12,
    color: '#666',
    marginLeft: 4,
  },
  votingBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#fff3cd',
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 20,
    alignSelf: 'flex-start',
  },
  votingText: {
    fontSize: 12,
    color: '#856404',
    marginLeft: 4,
    fontWeight: '600',
  },
  cartControls: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'flex-end',
  },
  cartButton: {
    backgroundColor: '#43e97b',
    width: 36,
    height: 36,
    borderRadius: 18,
    justifyContent: 'center',
    alignItems: 'center',
  },
  cartButtonDisabled: {
    backgroundColor: '#ccc',
  },
  cartQuantity: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#333',
    marginHorizontal: 15,
    minWidth: 20,
    textAlign: 'center',
  },
  cartSummary: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#43e97b',
    paddingHorizontal: 20,
    paddingVertical: 15,
    borderTopWidth: 1,
    borderTopColor: '#e0e0e0',
  },
  cartInfo: {
    flex: 1,
  },
  cartText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
  },
  orderButton: {
    backgroundColor: '#fff',
    paddingHorizontal: 20,
    paddingVertical: 10,
    borderRadius: 20,
  },
  orderButtonText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#43e97b',
  },
  emptyContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingVertical: 60,
  },
  emptyText: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#666',
    marginTop: 15,
  },
  emptySubtext: {
    fontSize: 14,
    color: '#999',
    marginTop: 5,
    textAlign: 'center',
  },
});

export default MenuScreen;

