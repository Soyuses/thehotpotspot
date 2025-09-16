import React, { useState } from 'react';
import {
  View,
  StyleSheet,
  KeyboardAvoidingView,
  Platform,
  ScrollView,
  Alert,
} from 'react-native';
import {
  Text,
  TextInput,
  Button,
  Card,
  Title,
  Paragraph,
  ActivityIndicator,
  Menu,
  Divider,
} from 'react-native-paper';
import { useAuth } from '../contexts/AuthContext';
import { useTheme } from '../contexts/ThemeContext';

const T_SHIRT_SIZES = ['XXS', 'XS', 'S', 'M', 'L', 'XL', 'XXL'];

const RegisterScreen: React.FC = () => {
  const [formData, setFormData] = useState({
    phone: '',
    name: '',
    email: '',
    tshirtSize: '',
    favoriteDish: '',
    password: '',
    confirmPassword: '',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [showSizeMenu, setShowSizeMenu] = useState(false);
  const { register } = useAuth();
  const { theme } = useTheme();

  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const validateForm = (): boolean => {
    if (!formData.phone.trim() || !formData.name.trim() || !formData.email.trim()) {
      Alert.alert('Ошибка', 'Пожалуйста, заполните все обязательные поля');
      return false;
    }

    if (formData.phone.length < 10) {
      Alert.alert('Ошибка', 'Номер телефона должен содержать минимум 10 цифр');
      return false;
    }

    if (!formData.email.includes('@')) {
      Alert.alert('Ошибка', 'Введите корректный email адрес');
      return false;
    }

    if (!formData.tshirtSize) {
      Alert.alert('Ошибка', 'Выберите размер футболки');
      return false;
    }

    if (!formData.favoriteDish.trim()) {
      Alert.alert('Ошибка', 'Укажите ваше любимое блюдо');
      return false;
    }

    if (formData.password.length < 8) {
      Alert.alert('Ошибка', 'Пароль должен содержать минимум 8 символов');
      return false;
    }

    if (formData.password !== formData.confirmPassword) {
      Alert.alert('Ошибка', 'Пароли не совпадают');
      return false;
    }

    return true;
  };

  const handleRegister = async () => {
    if (!validateForm()) return;

    try {
      setIsLoading(true);
      const success = await register(formData);
      
      if (success) {
        Alert.alert(
          'Успешно!', 
          'Регистрация завершена. Теперь вы можете войти в систему.',
          [{ text: 'OK', onPress: () => {
            // Navigate to login screen
            console.log('Navigate to login');
          }}]
        );
      } else {
        Alert.alert('Ошибка', 'Не удалось зарегистрироваться. Попробуйте еще раз.');
      }
    } catch (error) {
      Alert.alert('Ошибка', 'Произошла ошибка при регистрации');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <ScrollView contentContainerStyle={styles.scrollContent}>
        <View style={styles.header}>
          <Title style={[styles.title, { color: theme.colors.primary }]}>
            Регистрация
          </Title>
          <Paragraph style={[styles.subtitle, { color: theme.colors.onSurface }]}>
            Создайте новый аккаунт
          </Paragraph>
        </View>

        <Card style={[styles.card, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.cardContent}>
            <TextInput
              label="Номер телефона *"
              value={formData.phone}
              onChangeText={(value) => handleInputChange('phone', value)}
              mode="outlined"
              keyboardType="phone-pad"
              placeholder="+995 123 456 789"
              style={styles.input}
              disabled={isLoading}
            />

            <TextInput
              label="Полное имя *"
              value={formData.name}
              onChangeText={(value) => handleInputChange('name', value)}
              mode="outlined"
              placeholder="Иван Иванов"
              style={styles.input}
              disabled={isLoading}
            />

            <TextInput
              label="Email *"
              value={formData.email}
              onChangeText={(value) => handleInputChange('email', value)}
              mode="outlined"
              keyboardType="email-address"
              placeholder="ivan@example.com"
              style={styles.input}
              disabled={isLoading}
            />

            <View style={styles.sizeContainer}>
              <Text style={[styles.sizeLabel, { color: theme.colors.onSurface }]}>
                Размер футболки *
              </Text>
              <Menu
                visible={showSizeMenu}
                onDismiss={() => setShowSizeMenu(false)}
                anchor={
                  <Button
                    mode="outlined"
                    onPress={() => setShowSizeMenu(true)}
                    style={styles.sizeButton}
                    disabled={isLoading}
                  >
                    {formData.tshirtSize || 'Выберите размер'}
                  </Button>
                }
              >
                {T_SHIRT_SIZES.map((size) => (
                  <Menu.Item
                    key={size}
                    onPress={() => {
                      handleInputChange('tshirtSize', size);
                      setShowSizeMenu(false);
                    }}
                    title={size}
                  />
                ))}
              </Menu>
            </View>

            <TextInput
              label="Любимое блюдо *"
              value={formData.favoriteDish}
              onChangeText={(value) => handleInputChange('favoriteDish', value)}
              mode="outlined"
              placeholder="Плов, Хинкали, Хачапури..."
              style={styles.input}
              disabled={isLoading}
            />

            <Divider style={styles.divider} />

            <TextInput
              label="Пароль *"
              value={formData.password}
              onChangeText={(value) => handleInputChange('password', value)}
              mode="outlined"
              secureTextEntry
              placeholder="Минимум 8 символов"
              style={styles.input}
              disabled={isLoading}
            />

            <TextInput
              label="Подтвердите пароль *"
              value={formData.confirmPassword}
              onChangeText={(value) => handleInputChange('confirmPassword', value)}
              mode="outlined"
              secureTextEntry
              placeholder="Повторите пароль"
              style={styles.input}
              disabled={isLoading}
            />

            <Button
              mode="contained"
              onPress={handleRegister}
              style={[styles.registerButton, { backgroundColor: theme.colors.primary }]}
              contentStyle={styles.buttonContent}
              disabled={isLoading}
            >
              {isLoading ? (
                <ActivityIndicator color={theme.colors.onPrimary} size="small" />
              ) : (
                'Зарегистрироваться'
              )}
            </Button>
          </Card.Content>
        </Card>

        <View style={styles.footer}>
          <Paragraph style={[styles.footerText, { color: theme.colors.onSurface }]}>
            Уже есть аккаунт?{' '}
            <Text
              style={[styles.linkText, { color: theme.colors.primary }]}
              onPress={() => {
                // Navigate to login screen
                console.log('Navigate to login');
              }}
            >
              Войти
            </Text>
          </Paragraph>
        </View>
      </ScrollView>
    </KeyboardAvoidingView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  scrollContent: {
    flexGrow: 1,
    padding: 20,
    paddingTop: 40,
  },
  header: {
    alignItems: 'center',
    marginBottom: 30,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    textAlign: 'center',
  },
  card: {
    elevation: 4,
    borderRadius: 12,
  },
  cardContent: {
    padding: 20,
  },
  input: {
    marginBottom: 16,
  },
  sizeContainer: {
    marginBottom: 16,
  },
  sizeLabel: {
    fontSize: 16,
    marginBottom: 8,
  },
  sizeButton: {
    justifyContent: 'flex-start',
  },
  divider: {
    marginVertical: 16,
  },
  registerButton: {
    marginTop: 8,
    borderRadius: 8,
  },
  buttonContent: {
    paddingVertical: 8,
  },
  footer: {
    alignItems: 'center',
    marginTop: 20,
  },
  footerText: {
    fontSize: 14,
  },
  linkText: {
    fontWeight: 'bold',
  },
});

export default RegisterScreen;

