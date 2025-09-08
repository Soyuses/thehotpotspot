import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  RefreshControl,
  Alert,
  ActivityIndicator,
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import LinearGradient from 'react-native-linear-gradient';
import { LineChart } from 'react-native-chart-kit';
import { Dimensions } from 'react-native';

import useAppStore from '../store';

const { width } = Dimensions.get('window');

const WalletScreen: React.FC = () => {
  const [refreshing, setRefreshing] = useState(false);

  const { 
    user, 
    tokenBalance, 
    loadTokenBalance, 
    loadUserData,
    isLoading, 
    error 
  } = useAppStore();

  useEffect(() => {
    if (user) {
      loadTokenBalance(user.walletAddress);
    }
  }, [user]);

  const handleRefresh = async () => {
    setRefreshing(true);
    if (user) {
      await Promise.all([
        loadTokenBalance(user.walletAddress),
        loadUserData(user.walletAddress)
      ]);
    }
    setRefreshing(false);
  };

  const handleTransferFromCheck = () => {
    Alert.alert(
      'Перевод с чека',
      'Эта функция позволяет перевести токены с найденного чека на ваш кошелек',
      [
        { text: 'Отмена', style: 'cancel' },
        { text: 'Перейти', onPress: () => {
          // Навигация к экрану перевода с чека
          console.log('Navigate to check transfer');
        }}
      ]
    );
  };

  const handleViewHistory = () => {
    Alert.alert(
      'История операций',
      'Просмотр истории всех операций с токенами',
      [
        { text: 'OK', onPress: () => {
          // Навигация к истории операций
          console.log('Navigate to transaction history');
        }}
      ]
    );
  };

  // Демо-данные для графика
  const chartData = {
    labels: ['Пн', 'Вт', 'Ср', 'Чт', 'Пт', 'Сб', 'Вс'],
    datasets: [
      {
        data: [20, 45, 28, 80, 99, 43, 50],
        color: (opacity = 1) => `rgba(67, 233, 123, ${opacity})`,
        strokeWidth: 2,
      },
    ],
  };

  const chartConfig = {
    backgroundColor: '#ffffff',
    backgroundGradientFrom: '#ffffff',
    backgroundGradientTo: '#ffffff',
    decimalPlaces: 0,
    color: (opacity = 1) => `rgba(67, 233, 123, ${opacity})`,
    labelColor: (opacity = 1) => `rgba(0, 0, 0, ${opacity})`,
    style: {
      borderRadius: 16,
    },
    propsForDots: {
      r: '6',
      strokeWidth: '2',
      stroke: '#43e97b',
    },
  };

  if (isLoading && !tokenBalance) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#43e97b" />
        <Text style={styles.loadingText}>Загрузка баланса...</Text>
      </View>
    );
  }

  return (
    <ScrollView
      style={styles.container}
      refreshControl={
        <RefreshControl
          refreshing={refreshing}
          onRefresh={handleRefresh}
          colors={['#43e97b']}
        />
      }
    >
      {/* Заголовок с градиентом */}
      <LinearGradient
        colors={['#43e97b', '#38f9d7']}
        style={styles.header}
      >
        <View style={styles.headerContent}>
          <Icon name="account-balance-wallet" size={40} color="#fff" />
          <Text style={styles.headerTitle}>Мой кошелек</Text>
          <Text style={styles.headerSubtitle}>
            {user?.walletAddress ? `${user.walletAddress.slice(0, 6)}...${user.walletAddress.slice(-4)}` : 'Не авторизован'}
          </Text>
        </View>
      </LinearGradient>

      {/* Баланс токенов */}
      <View style={styles.balanceContainer}>
        <View style={styles.balanceCard}>
          <View style={styles.balanceHeader}>
            <Icon name="security" size={24} color="#43e97b" />
            <Text style={styles.balanceLabel}>Security Tokens</Text>
          </View>
          <Text style={styles.balanceValue}>
            {tokenBalance?.securityTokens.toFixed(2) || '0.00'}
          </Text>
        </View>

        <View style={styles.balanceCard}>
          <View style={styles.balanceHeader}>
            <Icon name="how-to-vote" size={24} color="#43e97b" />
            <Text style={styles.balanceLabel}>Utility Tokens</Text>
          </View>
          <Text style={styles.balanceValue}>
            {tokenBalance?.utilityTokens.toFixed(2) || '0.00'}
          </Text>
        </View>
      </View>

      {/* Общий баланс */}
      <View style={styles.totalBalanceCard}>
        <Text style={styles.totalBalanceLabel}>Общий баланс</Text>
        <Text style={styles.totalBalanceValue}>
          ${tokenBalance?.totalBalance.toFixed(2) || '0.00'}
        </Text>
        <Text style={styles.ownershipPercentage}>
          Владение: {tokenBalance?.ownershipPercentage.toFixed(2) || '0.00'}%
        </Text>
      </View>

      {/* График роста */}
      <View style={styles.chartContainer}>
        <Text style={styles.chartTitle}>Рост токенов (7 дней)</Text>
        <LineChart
          data={chartData}
          width={width - 40}
          height={220}
          chartConfig={chartConfig}
          bezier
          style={styles.chart}
        />
      </View>

      {/* Быстрые действия */}
      <View style={styles.actionsContainer}>
        <Text style={styles.actionsTitle}>Быстрые действия</Text>
        
        <TouchableOpacity style={styles.actionButton} onPress={handleTransferFromCheck}>
          <LinearGradient
            colors={['#43e97b', '#38f9d7']}
            style={styles.actionButtonGradient}
          >
            <Icon name="swap-horiz" size={24} color="#fff" />
            <Text style={styles.actionButtonText}>Перевод с чека</Text>
          </LinearGradient>
        </TouchableOpacity>

        <TouchableOpacity style={styles.actionButton} onPress={handleViewHistory}>
          <LinearGradient
            colors={['#667eea', '#764ba2']}
            style={styles.actionButtonGradient}
          >
            <Icon name="history" size={24} color="#fff" />
            <Text style={styles.actionButtonText}>История операций</Text>
          </LinearGradient>
        </TouchableOpacity>
      </View>

      {/* Статистика */}
      <View style={styles.statsContainer}>
        <Text style={styles.statsTitle}>Статистика</Text>
        
        <View style={styles.statsGrid}>
          <View style={styles.statItem}>
            <Icon name="trending-up" size={20} color="#43e97b" />
            <Text style={styles.statValue}>+12.5%</Text>
            <Text style={styles.statLabel}>За неделю</Text>
          </View>
          
          <View style={styles.statItem}>
            <Icon name="receipt" size={20} color="#43e97b" />
            <Text style={styles.statValue}>24</Text>
            <Text style={styles.statLabel}>Заказов</Text>
          </View>
          
          <View style={styles.statItem}>
            <Icon name="how-to-vote" size={20} color="#43e97b" />
            <Text style={styles.statValue}>8</Text>
            <Text style={styles.statLabel}>Голосов</Text>
          </View>
        </View>
      </View>
    </ScrollView>
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
  header: {
    padding: 30,
    paddingTop: 50,
  },
  headerContent: {
    alignItems: 'center',
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#fff',
    marginTop: 10,
  },
  headerSubtitle: {
    fontSize: 14,
    color: '#fff',
    opacity: 0.9,
    marginTop: 5,
  },
  balanceContainer: {
    flexDirection: 'row',
    padding: 20,
    gap: 15,
  },
  balanceCard: {
    flex: 1,
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 20,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  balanceHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 10,
  },
  balanceLabel: {
    fontSize: 14,
    color: '#666',
    marginLeft: 8,
  },
  balanceValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#43e97b',
  },
  totalBalanceCard: {
    backgroundColor: '#fff',
    margin: 20,
    marginTop: 0,
    borderRadius: 12,
    padding: 25,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  totalBalanceLabel: {
    fontSize: 16,
    color: '#666',
    marginBottom: 5,
  },
  totalBalanceValue: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#43e97b',
    marginBottom: 5,
  },
  ownershipPercentage: {
    fontSize: 14,
    color: '#999',
  },
  chartContainer: {
    backgroundColor: '#fff',
    margin: 20,
    marginTop: 0,
    borderRadius: 12,
    padding: 20,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  chartTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 15,
    textAlign: 'center',
  },
  chart: {
    borderRadius: 16,
  },
  actionsContainer: {
    padding: 20,
    paddingTop: 0,
  },
  actionsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 15,
  },
  actionButton: {
    marginBottom: 15,
    borderRadius: 12,
    overflow: 'hidden',
  },
  actionButtonGradient: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 15,
    paddingHorizontal: 20,
  },
  actionButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
    marginLeft: 10,
  },
  statsContainer: {
    backgroundColor: '#fff',
    margin: 20,
    marginTop: 0,
    borderRadius: 12,
    padding: 20,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  statsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 15,
  },
  statsGrid: {
    flexDirection: 'row',
    justifyContent: 'space-around',
  },
  statItem: {
    alignItems: 'center',
  },
  statValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#43e97b',
    marginTop: 5,
  },
  statLabel: {
    fontSize: 12,
    color: '#666',
    marginTop: 2,
  },
});

export default WalletScreen;

