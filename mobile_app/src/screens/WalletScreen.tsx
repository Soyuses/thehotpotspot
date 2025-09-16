import React, { useState, useEffect } from 'react';
import {
  View,
  StyleSheet,
  ScrollView,
  RefreshControl,
  Alert,
} from 'react-native';
import {
  Text,
  Card,
  Title,
  Paragraph,
  Button,
  List,
  Divider,
  ActivityIndicator,
  FAB,
} from 'react-native-paper';
import { useWallet } from '../contexts/WalletContext';
import { useTheme } from '../contexts/ThemeContext';

const WalletScreen: React.FC = () => {
  const [refreshing, setRefreshing] = useState(false);
  const [showTransferDialog, setShowTransferDialog] = useState(false);
  const { balance, transactions, isLoading, refreshBalance, refreshTransactions } = useWallet();
  const { theme } = useTheme();

  useEffect(() => {
    loadWalletData();
  }, []);

  const loadWalletData = async () => {
    await Promise.all([
      refreshBalance(),
      refreshTransactions(),
    ]);
  };

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadWalletData();
    setRefreshing(false);
  };

  const formatAmount = (amount: number) => {
    return amount.toLocaleString('ru-RU', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('ru-RU', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getTransactionIcon = (type: string) => {
    switch (type) {
      case 'st_received':
        return 'arrow-down-bold';
      case 'ut_earned':
        return 'star';
      case 'st_transferred':
        return 'arrow-up-bold';
      case 'dividend_received':
        return 'cash';
      default:
        return 'circle';
    }
  };

  const getTransactionColor = (type: string) => {
    switch (type) {
      case 'st_received':
      case 'ut_earned':
      case 'dividend_received':
        return theme.colors.primary;
      case 'st_transferred':
        return theme.colors.error;
      default:
        return theme.colors.onSurface;
    }
  };

  const getTransactionTitle = (type: string) => {
    switch (type) {
      case 'st_received':
        return 'Получены ST токены';
      case 'ut_earned':
        return 'Заработаны UT токены';
      case 'st_transferred':
        return 'Переведены ST токены';
      case 'dividend_received':
        return 'Получены дивиденды';
      default:
        return 'Транзакция';
    }
  };

  return (
    <View style={styles.container}>
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={handleRefresh} />
        }
      >
        {/* Balance Cards */}
        <View style={styles.balanceSection}>
          <Card style={[styles.balanceCard, { backgroundColor: theme.colors.primary }]}>
            <Card.Content style={styles.balanceContent}>
              <Title style={[styles.balanceTitle, { color: theme.colors.onPrimary }]}>
                Security Tokens (ST)
              </Title>
              <Text style={[styles.balanceAmount, { color: theme.colors.onPrimary }]}>
                {formatAmount(balance.stTokens)}
              </Text>
              <Paragraph style={[styles.balanceValue, { color: theme.colors.onPrimary }]}>
                ≈ {formatAmount(balance.stTokens * 0.05)} GEL
              </Paragraph>
            </Card.Content>
          </Card>

          <Card style={[styles.balanceCard, { backgroundColor: theme.colors.secondary }]}>
            <Card.Content style={styles.balanceContent}>
              <Title style={[styles.balanceTitle, { color: theme.colors.onSecondary }]}>
                Utility Tokens (UT)
              </Title>
              <Text style={[styles.balanceAmount, { color: theme.colors.onSecondary }]}>
                {formatAmount(balance.utTokens)}
              </Text>
              <Paragraph style={[styles.balanceValue, { color: theme.colors.onSecondary }]}>
                Голосование в DAO
              </Paragraph>
            </Card.Content>
          </Card>
        </View>

        {/* Quick Actions */}
        <Card style={[styles.actionsCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.actionsContent}>
            <Title style={[styles.actionsTitle, { color: theme.colors.onSurface }]}>
              Быстрые действия
            </Title>
            <View style={styles.actionsRow}>
              <Button
                mode="outlined"
                onPress={() => {
                  // Navigate to QR scanner
                  console.log('Navigate to QR scanner');
                }}
                style={styles.actionButton}
                icon="qrcode-scan"
              >
                Сканировать QR
              </Button>
              <Button
                mode="outlined"
                onPress={() => setShowTransferDialog(true)}
                style={styles.actionButton}
                icon="send"
              >
                Перевести
              </Button>
            </View>
          </Card.Content>
        </Card>

        {/* Transaction History */}
        <Card style={[styles.transactionsCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.transactionsContent}>
            <Title style={[styles.transactionsTitle, { color: theme.colors.onSurface }]}>
              История транзакций
            </Title>
            
            {isLoading ? (
              <View style={styles.loadingContainer}>
                <ActivityIndicator size="large" color={theme.colors.primary} />
                <Text style={[styles.loadingText, { color: theme.colors.onSurface }]}>
                  Загрузка транзакций...
                </Text>
              </View>
            ) : transactions.length === 0 ? (
              <View style={styles.emptyContainer}>
                <Text style={[styles.emptyText, { color: theme.colors.onSurface }]}>
                  Пока нет транзакций
                </Text>
                <Paragraph style={[styles.emptySubtext, { color: theme.colors.onSurface }]}>
                  Сканируйте QR-код с чека, чтобы получить первые токены
                </Paragraph>
              </View>
            ) : (
              <List.Section>
                {transactions.map((transaction, index) => (
                  <React.Fragment key={transaction.id}>
                    <List.Item
                      title={getTransactionTitle(transaction.type)}
                      description={transaction.description}
                      left={(props) => (
                        <List.Icon
                          {...props}
                          icon={getTransactionIcon(transaction.type)}
                          color={getTransactionColor(transaction.type)}
                        />
                      )}
                      right={(props) => (
                        <View style={styles.transactionRight}>
                          <Text
                            style={[
                              styles.transactionAmount,
                              { color: getTransactionColor(transaction.type) },
                            ]}
                          >
                            {transaction.type === 'st_transferred' ? '-' : '+'}
                            {formatAmount(transaction.amount)}
                          </Text>
                          <Text style={[styles.transactionDate, { color: theme.colors.onSurface }]}>
                            {formatDate(transaction.timestamp)}
                          </Text>
                        </View>
                      )}
                    />
                    {index < transactions.length - 1 && <Divider />}
                  </React.Fragment>
                ))}
              </List.Section>
            )}
          </Card.Content>
        </Card>
      </ScrollView>

      {/* Floating Action Button */}
      <FAB
        icon="qrcode-scan"
        style={[styles.fab, { backgroundColor: theme.colors.primary }]}
        onPress={() => {
          // Navigate to QR scanner
          console.log('Navigate to QR scanner');
        }}
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  scrollView: {
    flex: 1,
  },
  balanceSection: {
    flexDirection: 'row',
    padding: 16,
    gap: 12,
  },
  balanceCard: {
    flex: 1,
    elevation: 4,
    borderRadius: 12,
  },
  balanceContent: {
    alignItems: 'center',
    padding: 16,
  },
  balanceTitle: {
    fontSize: 14,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  balanceAmount: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  balanceValue: {
    fontSize: 12,
    opacity: 0.8,
  },
  actionsCard: {
    margin: 16,
    marginTop: 0,
    elevation: 2,
    borderRadius: 12,
  },
  actionsContent: {
    padding: 16,
  },
  actionsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 12,
  },
  actionsRow: {
    flexDirection: 'row',
    gap: 12,
  },
  actionButton: {
    flex: 1,
    borderRadius: 8,
  },
  transactionsCard: {
    margin: 16,
    marginTop: 0,
    elevation: 2,
    borderRadius: 12,
  },
  transactionsContent: {
    padding: 16,
  },
  transactionsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 12,
  },
  loadingContainer: {
    alignItems: 'center',
    padding: 20,
  },
  loadingText: {
    marginTop: 12,
    fontSize: 14,
  },
  emptyContainer: {
    alignItems: 'center',
    padding: 20,
  },
  emptyText: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  emptySubtext: {
    fontSize: 14,
    textAlign: 'center',
    lineHeight: 20,
  },
  transactionRight: {
    alignItems: 'flex-end',
  },
  transactionAmount: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  transactionDate: {
    fontSize: 12,
    opacity: 0.7,
  },
  fab: {
    position: 'absolute',
    margin: 16,
    right: 0,
    bottom: 0,
  },
});

export default WalletScreen;

