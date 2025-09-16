import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { createStackNavigator } from '@react-navigation/stack';
import { Provider as PaperProvider } from 'react-native-paper';
import { QueryClient, QueryClientProvider } from 'react-query';
import Toast from 'react-native-toast-message';
import Icon from 'react-native-vector-icons/MaterialIcons';

// Screens
import LoginScreen from './src/screens/LoginScreen';
import RegisterScreen from './src/screens/RegisterScreen';
import QRScannerScreen from './src/screens/QRScannerScreen';
import WalletScreen from './src/screens/WalletScreen';
import MenuScreen from './src/screens/MenuScreen';
import VotingScreen from './src/screens/VotingScreen';
import ProfileScreen from './src/screens/ProfileScreen';
import CheckClaimScreen from './src/screens/CheckClaimScreen';

// Services
import { AuthProvider, useAuth } from './src/contexts/AuthContext';
import { WalletProvider } from './src/contexts/WalletContext';

// Theme
import { theme } from './src/theme/theme';

const Tab = createBottomTabNavigator();
const Stack = createStackNavigator();
const queryClient = new QueryClient();

// Main Tab Navigator
function MainTabs() {
  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: string;

          switch (route.name) {
            case 'Wallet':
              iconName = 'account-balance-wallet';
              break;
            case 'Menu':
              iconName = 'restaurant-menu';
              break;
            case 'Voting':
              iconName = 'how-to-vote';
              break;
            case 'Profile':
              iconName = 'person';
              break;
            default:
              iconName = 'help';
          }

          return <Icon name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: theme.colors.primary,
        tabBarInactiveTintColor: 'gray',
        headerStyle: {
          backgroundColor: theme.colors.primary,
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      })}
    >
      <Tab.Screen 
        name="Wallet" 
        component={WalletScreen}
        options={{ title: 'Кошелек' }}
      />
      <Tab.Screen 
        name="Menu" 
        component={MenuScreen}
        options={{ title: 'Меню' }}
      />
      <Tab.Screen 
        name="Voting" 
        component={VotingScreen}
        options={{ title: 'Голосование' }}
      />
      <Tab.Screen 
        name="Profile" 
        component={ProfileScreen}
        options={{ title: 'Профиль' }}
      />
    </Tab.Navigator>
  );
}

// Auth Stack Navigator
function AuthStack() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerStyle: {
          backgroundColor: theme.colors.primary,
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      }}
    >
      <Stack.Screen 
        name="Login" 
        component={LoginScreen}
        options={{ title: 'Вход в The Hot Pot Spot' }}
      />
      <Stack.Screen 
        name="Register" 
        component={RegisterScreen}
        options={{ title: 'Регистрация' }}
      />
      <Stack.Screen 
        name="QRScanner" 
        component={QRScannerScreen}
        options={{ title: 'Сканирование QR-кода' }}
      />
      <Stack.Screen 
        name="CheckClaim" 
        component={CheckClaimScreen}
        options={{ title: 'Активация чека' }}
      />
    </Stack.Navigator>
  );
}

// Main App Navigator
function AppNavigator() {
  const { isAuthenticated } = useAuth();

  return (
    <NavigationContainer>
      {isAuthenticated ? <MainTabs /> : <AuthStack />}
    </NavigationContainer>
  );
}

// Main App Component
export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <PaperProvider theme={theme}>
        <AuthProvider>
          <WalletProvider>
            <AppNavigator />
            <Toast />
          </WalletProvider>
        </AuthProvider>
      </PaperProvider>
    </QueryClientProvider>
  );
}