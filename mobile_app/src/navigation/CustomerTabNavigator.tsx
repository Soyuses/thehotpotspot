import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import Icon from 'react-native-vector-icons/MaterialIcons';

import { CustomerTabParamList } from '../types';

// Screens
import MenuScreen from '../screens/customer/MenuScreen';
import OrdersScreen from '../screens/customer/OrdersScreen';
import VotingScreen from '../screens/customer/VotingScreen';
import WalletScreen from '../screens/customer/WalletScreen';
import ProfileScreen from '../screens/ProfileScreen';

const Tab = createBottomTabNavigator<CustomerTabParamList>();

const CustomerTabNavigator: React.FC = () => {
  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: string;

          switch (route.name) {
            case 'Menu':
              iconName = 'restaurant-menu';
              break;
            case 'Orders':
              iconName = 'receipt';
              break;
            case 'Voting':
              iconName = 'how-to-vote';
              break;
            case 'Wallet':
              iconName = 'account-balance-wallet';
              break;
            case 'Profile':
              iconName = 'person';
              break;
            default:
              iconName = 'help';
          }

          return <Icon name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: '#43e97b',
        tabBarInactiveTintColor: 'gray',
        tabBarStyle: {
          backgroundColor: '#fff',
          borderTopWidth: 1,
          borderTopColor: '#e0e0e0',
          paddingBottom: 5,
          paddingTop: 5,
          height: 60,
        },
        headerStyle: {
          backgroundColor: '#43e97b',
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      })}
    >
      <Tab.Screen
        name="Menu"
        component={MenuScreen}
        options={{
          title: 'Меню',
          headerTitle: '🍽️ Прейскурант',
        }}
      />
      <Tab.Screen
        name="Orders"
        component={OrdersScreen}
        options={{
          title: 'Заказы',
          headerTitle: '📋 Мои заказы',
        }}
      />
      <Tab.Screen
        name="Voting"
        component={VotingScreen}
        options={{
          title: 'Голосование',
          headerTitle: '🗳️ Голосование',
        }}
      />
      <Tab.Screen
        name="Wallet"
        component={WalletScreen}
        options={{
          title: 'Кошелек',
          headerTitle: '💰 Кошелек',
        }}
      />
      <Tab.Screen
        name="Profile"
        component={ProfileScreen}
        options={{
          title: 'Профиль',
          headerTitle: '👤 Профиль',
        }}
      />
    </Tab.Navigator>
  );
};

export default CustomerTabNavigator;
