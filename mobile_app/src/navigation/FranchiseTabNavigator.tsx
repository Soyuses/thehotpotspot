import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import Icon from 'react-native-vector-icons/MaterialIcons';

import { FranchiseTabParamList } from '../types';

// Screens
import OverviewScreen from '../screens/franchise/OverviewScreen';
import OrdersScreen from '../screens/franchise/OrdersScreen';
import AnalyticsScreen from '../screens/franchise/AnalyticsScreen';
import SettingsScreen from '../screens/SettingsScreen';

const Tab = createBottomTabNavigator<FranchiseTabParamList>();

const FranchiseTabNavigator: React.FC = () => {
  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: string;

          switch (route.name) {
            case 'Overview':
              iconName = 'dashboard';
              break;
            case 'Orders':
              iconName = 'receipt';
              break;
            case 'Analytics':
              iconName = 'analytics';
              break;
            case 'Settings':
              iconName = 'settings';
              break;
            default:
              iconName = 'help';
          }

          return <Icon name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: '#4facfe',
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
          backgroundColor: '#4facfe',
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      })}
    >
      <Tab.Screen
        name="Overview"
        component={OverviewScreen}
        options={{
          title: 'Обзор',
          headerTitle: '🏪 Обзор франшизы',
        }}
      />
      <Tab.Screen
        name="Orders"
        component={OrdersScreen}
        options={{
          title: 'Заказы',
          headerTitle: '📋 Управление заказами',
        }}
      />
      <Tab.Screen
        name="Analytics"
        component={AnalyticsScreen}
        options={{
          title: 'Аналитика',
          headerTitle: '📊 Аналитика франшизы',
        }}
      />
      <Tab.Screen
        name="Settings"
        component={SettingsScreen}
        options={{
          title: 'Настройки',
          headerTitle: '⚙️ Настройки',
        }}
      />
    </Tab.Navigator>
  );
};

export default FranchiseTabNavigator;
