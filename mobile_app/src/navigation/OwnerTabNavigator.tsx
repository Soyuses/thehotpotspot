import React from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import Icon from 'react-native-vector-icons/MaterialIcons';

import { OwnerTabParamList } from '../types';

// Screens
import DashboardScreen from '../screens/owner/DashboardScreen';
import FranchisesScreen from '../screens/owner/FranchisesScreen';
import AlertsScreen from '../screens/owner/AlertsScreen';
import AnalyticsScreen from '../screens/owner/AnalyticsScreen';
import SettingsScreen from '../screens/SettingsScreen';

const Tab = createBottomTabNavigator<OwnerTabParamList>();

const OwnerTabNavigator: React.FC = () => {
  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: string;

          switch (route.name) {
            case 'Dashboard':
              iconName = 'dashboard';
              break;
            case 'Franchises':
              iconName = 'store';
              break;
            case 'Alerts':
              iconName = 'notifications';
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
        tabBarActiveTintColor: '#ff6b6b',
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
          backgroundColor: '#ff6b6b',
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      })}
    >
      <Tab.Screen
        name="Dashboard"
        component={DashboardScreen}
        options={{
          title: 'Дашборд',
          headerTitle: '👑 Дашборд владельца',
        }}
      />
      <Tab.Screen
        name="Franchises"
        component={FranchisesScreen}
        options={{
          title: 'Франшизы',
          headerTitle: '🏪 Управление франшизами',
        }}
      />
      <Tab.Screen
        name="Alerts"
        component={AlertsScreen}
        options={{
          title: 'Алерты',
          headerTitle: '⚠️ Мониторинг',
        }}
      />
      <Tab.Screen
        name="Analytics"
        component={AnalyticsScreen}
        options={{
          title: 'Аналитика',
          headerTitle: '📊 Аналитика сети',
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

export default OwnerTabNavigator;
