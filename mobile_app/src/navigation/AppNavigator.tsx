import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import Icon from 'react-native-vector-icons/MaterialIcons';

import useAppStore from '../store';
import { RootStackParamList, CustomerTabParamList, OwnerTabParamList, FranchiseTabParamList } from '../types';

// Screens
import LoginScreen from '../screens/LoginScreen';
import CustomerTabNavigator from './CustomerTabNavigator';
import OwnerTabNavigator from './OwnerTabNavigator';
import FranchiseTabNavigator from './FranchiseTabNavigator';
import OrderDetailsScreen from '../screens/OrderDetailsScreen';
import MenuItemDetailsScreen from '../screens/MenuItemDetailsScreen';
import VotingDetailsScreen from '../screens/VotingDetailsScreen';
import CheckTransferScreen from '../screens/CheckTransferScreen';
import ProfileScreen from '../screens/ProfileScreen';
import SettingsScreen from '../screens/SettingsScreen';

const Stack = createStackNavigator<RootStackParamList>();

const AppNavigator: React.FC = () => {
  const { user } = useAppStore();

  return (
    <NavigationContainer>
      <Stack.Navigator
        screenOptions={{
          headerStyle: {
            backgroundColor: '#667eea',
          },
          headerTintColor: '#fff',
          headerTitleStyle: {
            fontWeight: 'bold',
          },
        }}
      >
        {!user ? (
          // Пользователь не авторизован
          <Stack.Screen
            name="Login"
            component={LoginScreen}
            options={{
              title: 'Blockchain Food Truck',
              headerShown: false,
            }}
          />
        ) : (
          // Пользователь авторизован
          <>
            <Stack.Screen
              name="Main"
              component={getMainNavigator(user.role)}
              options={{
                title: getMainTitle(user.role),
                headerShown: false,
              }}
            />
            <Stack.Screen
              name="OrderDetails"
              component={OrderDetailsScreen}
              options={{
                title: 'Детали заказа',
              }}
            />
            <Stack.Screen
              name="MenuItemDetails"
              component={MenuItemDetailsScreen}
              options={{
                title: 'Детали блюда',
              }}
            />
            <Stack.Screen
              name="VotingDetails"
              component={VotingDetailsScreen}
              options={{
                title: 'Голосование',
              }}
            />
            <Stack.Screen
              name="CheckTransfer"
              component={CheckTransferScreen}
              options={{
                title: 'Перевод с чека',
              }}
            />
            <Stack.Screen
              name="Profile"
              component={ProfileScreen}
              options={{
                title: 'Профиль',
              }}
            />
            <Stack.Screen
              name="Settings"
              component={SettingsScreen}
              options={{
                title: 'Настройки',
              }}
            />
          </>
        )}
      </Stack.Navigator>
    </NavigationContainer>
  );
};

const getMainNavigator = (role: string) => {
  switch (role) {
    case 'owner':
      return OwnerTabNavigator;
    case 'franchise':
      return FranchiseTabNavigator;
    case 'customer':
    default:
      return CustomerTabNavigator;
  }
};

const getMainTitle = (role: string) => {
  switch (role) {
    case 'owner':
      return 'Владелец сети';
    case 'franchise':
      return 'Владелец франшизы';
    case 'customer':
    default:
      return 'Покупатель';
  }
};

export default AppNavigator;
