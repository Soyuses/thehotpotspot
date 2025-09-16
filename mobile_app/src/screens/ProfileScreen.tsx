import React, { useState, useEffect } from 'react';
import {
  View,
  StyleSheet,
  ScrollView,
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
  Chip,
  Avatar,
} from 'react-native-paper';
import { useAuth } from '../contexts/AuthContext';
import { useTheme } from '../contexts/ThemeContext';

const ProfileScreen: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [kycStatus, setKycStatus] = useState<'pending' | 'approved' | 'rejected'>('pending');
  const { user, logout, updateProfile, checkKYCStatus } = useAuth();
  const { theme } = useTheme();

  useEffect(() => {
    if (user) {
      checkKYCStatus();
      setKycStatus(user.kycStatus);
    }
  }, [user]);

  const handleLogout = async () => {
    Alert.alert(
      'Выход',
      'Вы уверены, что хотите выйти из аккаунта?',
      [
        { text: 'Отмена', style: 'cancel' },
        {
          text: 'Выйти',
          style: 'destructive',
          onPress: async () => {
            await logout();
          },
        },
      ]
    );
  };

  const handleKYCRequest = async () => {
    if (kycStatus === 'approved') {
      Alert.alert('KYC уже пройден', 'Ваша личность уже подтверждена');
      return;
    }

    Alert.alert(
      'Прохождение KYC',
      'Для участия в распределении Security токенов необходимо пройти процедуру KYC (Know Your Customer). Это включает в себя предоставление документов, удостоверяющих личность.',
      [
        { text: 'Отмена', style: 'cancel' },
        {
          text: 'Начать KYC',
          onPress: () => {
            // Navigate to KYC screen
            console.log('Navigate to KYC screen');
          },
        },
      ]
    );
  };

  const getKYCStatusColor = (status: string) => {
    switch (status) {
      case 'approved':
        return '#4CAF50';
      case 'rejected':
        return theme.colors.error;
      case 'pending':
        return '#FF9800';
      default:
        return theme.colors.onSurface;
    }
  };

  const getKYCStatusLabel = (status: string) => {
    switch (status) {
      case 'approved':
        return 'Подтверждено';
      case 'rejected':
        return 'Отклонено';
      case 'pending':
        return 'На рассмотрении';
      default:
        return 'Неизвестно';
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('ru-RU', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    });
  };

  if (!user) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={theme.colors.primary} />
        <Text style={[styles.loadingText, { color: theme.colors.onSurface }]}>
          Загрузка профиля...
        </Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Profile Header */}
        <Card style={[styles.profileCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.profileContent}>
            <View style={styles.profileHeader}>
              <Avatar.Text
                size={80}
                label={user.name.charAt(0).toUpperCase()}
                style={{ backgroundColor: theme.colors.primary }}
              />
              <View style={styles.profileInfo}>
                <Title style={[styles.profileName, { color: theme.colors.onSurface }]}>
                  {user.name}
                </Title>
                <Paragraph style={[styles.profilePhone, { color: theme.colors.onSurface }]}>
                  {user.phone}
                </Paragraph>
                <Paragraph style={[styles.profileEmail, { color: theme.colors.onSurface }]}>
                  {user.email}
                </Paragraph>
              </View>
            </View>
            
            <View style={styles.kycStatus}>
              <Chip
                style={[styles.kycChip, { backgroundColor: getKYCStatusColor(kycStatus) }]}
                textStyle={{ color: theme.colors.onPrimary }}
              >
                KYC: {getKYCStatusLabel(kycStatus)}
              </Chip>
            </View>
          </Card.Content>
        </Card>

        {/* Personal Information */}
        <Card style={[styles.infoCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.infoContent}>
            <Title style={[styles.infoTitle, { color: theme.colors.onSurface }]}>
              Личная информация
            </Title>
            
            <List.Section>
              <List.Item
                title="Размер футболки"
                description={user.tshirtSize}
                left={(props) => <List.Icon {...props} icon="tshirt-crew" />}
              />
              <Divider />
              <List.Item
                title="Любимое блюдо"
                description={user.favoriteDish}
                left={(props) => <List.Icon {...props} icon="food" />}
              />
              <Divider />
              <List.Item
                title="Дата регистрации"
                description={formatDate(user.createdAt)}
                left={(props) => <List.Icon {...props} icon="calendar" />}
              />
            </List.Section>
          </Card.Content>
        </Card>

        {/* KYC Section */}
        <Card style={[styles.kycCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.kycContent}>
            <Title style={[styles.kycTitle, { color: theme.colors.onSurface }]}>
              KYC Verification
            </Title>
            
            <Paragraph style={[styles.kycDescription, { color: theme.colors.onSurface }]}>
              {kycStatus === 'approved' 
                ? 'Ваша личность подтверждена. Вы можете участвовать в распределении Security токенов.'
                : kycStatus === 'rejected'
                ? 'Ваша заявка на KYC была отклонена. Обратитесь в поддержку для получения дополнительной информации.'
                : 'Для участия в распределении Security токенов необходимо пройти процедуру KYC.'
              }
            </Paragraph>

            {kycStatus !== 'approved' && (
              <Button
                mode="contained"
                onPress={handleKYCRequest}
                style={[styles.kycButton, { backgroundColor: theme.colors.primary }]}
                disabled={isLoading}
              >
                {isLoading ? (
                  <ActivityIndicator color={theme.colors.onPrimary} size="small" />
                ) : (
                  'Пройти KYC'
                )}
              </Button>
            )}
          </Card.Content>
        </Card>

        {/* Settings */}
        <Card style={[styles.settingsCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.settingsContent}>
            <Title style={[styles.settingsTitle, { color: theme.colors.onSurface }]}>
              Настройки
            </Title>
            
            <List.Section>
              <List.Item
                title="Редактировать профиль"
                description="Изменить личную информацию"
                left={(props) => <List.Icon {...props} icon="account-edit" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => {
                  // Navigate to edit profile screen
                  console.log('Navigate to edit profile');
                }}
              />
              <Divider />
              <List.Item
                title="Уведомления"
                description="Настройки уведомлений"
                left={(props) => <List.Icon {...props} icon="bell" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => {
                  // Navigate to notifications settings
                  console.log('Navigate to notifications settings');
                }}
              />
              <Divider />
              <List.Item
                title="Безопасность"
                description="Пароль и безопасность"
                left={(props) => <List.Icon {...props} icon="shield-account" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => {
                  // Navigate to security settings
                  console.log('Navigate to security settings');
                }}
              />
              <Divider />
              <List.Item
                title="Поддержка"
                description="Связаться с поддержкой"
                left={(props) => <List.Icon {...props} icon="help-circle" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => {
                  // Navigate to support
                  console.log('Navigate to support');
                }}
              />
            </List.Section>
          </Card.Content>
        </Card>

        {/* Logout Button */}
        <Card style={[styles.logoutCard, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.logoutContent}>
            <Button
              mode="outlined"
              onPress={handleLogout}
              style={[styles.logoutButton, { borderColor: theme.colors.error }]}
              textColor={theme.colors.error}
              icon="logout"
            >
              Выйти из аккаунта
            </Button>
          </Card.Content>
        </Card>
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
  profileCard: {
    margin: 16,
    marginBottom: 8,
    elevation: 2,
    borderRadius: 12,
  },
  profileContent: {
    padding: 20,
  },
  profileHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 16,
  },
  profileInfo: {
    flex: 1,
    marginLeft: 16,
  },
  profileName: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  profilePhone: {
    fontSize: 14,
    marginBottom: 2,
  },
  profileEmail: {
    fontSize: 14,
  },
  kycStatus: {
    alignItems: 'flex-start',
  },
  kycChip: {
    borderRadius: 16,
  },
  infoCard: {
    margin: 16,
    marginTop: 8,
    elevation: 2,
    borderRadius: 12,
  },
  infoContent: {
    padding: 16,
  },
  infoTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 12,
  },
  kycCard: {
    margin: 16,
    marginTop: 8,
    elevation: 2,
    borderRadius: 12,
  },
  kycContent: {
    padding: 16,
  },
  kycTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  kycDescription: {
    fontSize: 14,
    lineHeight: 20,
    marginBottom: 16,
  },
  kycButton: {
    borderRadius: 8,
  },
  settingsCard: {
    margin: 16,
    marginTop: 8,
    elevation: 2,
    borderRadius: 12,
  },
  settingsContent: {
    padding: 16,
  },
  settingsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 12,
  },
  logoutCard: {
    margin: 16,
    marginTop: 8,
    elevation: 2,
    borderRadius: 12,
  },
  logoutContent: {
    padding: 16,
  },
  logoutButton: {
    borderRadius: 8,
  },
});

export default ProfileScreen;

