import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  Alert,
  KeyboardAvoidingView,
  Platform,
  ScrollView,
  ActivityIndicator,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/MaterialIcons';

import useAppStore from '../store';

const LoginScreen: React.FC = () => {
  const [phoneNumber, setPhoneNumber] = useState('');
  const [walletAddress, setWalletAddress] = useState('');
  const [verificationCode, setVerificationCode] = useState('');
  const [isVerifying, setIsVerifying] = useState(false);
  const [isRegistering, setIsRegistering] = useState(false);

  const { registerUser, verifyUser, isLoading, error } = useAppStore();

  const handleRegister = async () => {
    if (!phoneNumber.trim() || !walletAddress.trim()) {
      Alert.alert('Ошибка', 'Пожалуйста, заполните все поля');
      return;
    }

    setIsRegistering(true);
    const success = await registerUser(phoneNumber.trim(), walletAddress.trim());
    setIsRegistering(false);

    if (success) {
      Alert.alert(
        'Успешно!',
        'Код подтверждения отправлен на ваш номер телефона',
        [{ text: 'OK' }]
      );
      setIsVerifying(true);
    } else {
      Alert.alert('Ошибка', error || 'Не удалось зарегистрироваться');
    }
  };

  const handleVerify = async () => {
    if (!verificationCode.trim()) {
      Alert.alert('Ошибка', 'Пожалуйста, введите код подтверждения');
      return;
    }

    const success = await verifyUser(phoneNumber.trim(), verificationCode.trim());
    
    if (success) {
      Alert.alert('Успешно!', 'Вы успешно авторизованы в системе');
    } else {
      Alert.alert('Ошибка', error || 'Неверный код подтверждения');
    }
  };

  const handleBackToRegister = () => {
    setIsVerifying(false);
    setVerificationCode('');
  };

  return (
    <LinearGradient
      colors={['#667eea', '#764ba2']}
      style={styles.container}
    >
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
        style={styles.keyboardAvoidingView}
      >
        <ScrollView contentContainerStyle={styles.scrollContainer}>
          <View style={styles.header}>
            <Icon name="restaurant" size={80} color="#fff" />
            <Text style={styles.title}>Blockchain Food Truck</Text>
            <Text style={styles.subtitle}>Децентрализованная сеть фудтраков</Text>
          </View>

          <View style={styles.formContainer}>
            {!isVerifying ? (
              // Форма регистрации
              <>
                <Text style={styles.formTitle}>Регистрация</Text>
                
                <View style={styles.inputContainer}>
                  <Icon name="phone" size={24} color="#667eea" style={styles.inputIcon} />
                  <TextInput
                    style={styles.input}
                    placeholder="Номер телефона"
                    placeholderTextColor="#999"
                    value={phoneNumber}
                    onChangeText={setPhoneNumber}
                    keyboardType="phone-pad"
                    autoCapitalize="none"
                  />
                </View>

                <View style={styles.inputContainer}>
                  <Icon name="account-balance-wallet" size={24} color="#667eea" style={styles.inputIcon} />
                  <TextInput
                    style={styles.input}
                    placeholder="Адрес кошелька"
                    placeholderTextColor="#999"
                    value={walletAddress}
                    onChangeText={setWalletAddress}
                    autoCapitalize="none"
                    multiline
                  />
                </View>

                <TouchableOpacity
                  style={[styles.button, isLoading && styles.buttonDisabled]}
                  onPress={handleRegister}
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <ActivityIndicator color="#fff" />
                  ) : (
                    <>
                      <Icon name="person-add" size={24} color="#fff" />
                      <Text style={styles.buttonText}>Зарегистрироваться</Text>
                    </>
                  )}
                </TouchableOpacity>
              </>
            ) : (
              // Форма верификации
              <>
                <Text style={styles.formTitle}>Подтверждение</Text>
                <Text style={styles.verificationText}>
                  Введите код подтверждения, отправленный на номер {phoneNumber}
                </Text>

                <View style={styles.inputContainer}>
                  <Icon name="security" size={24} color="#667eea" style={styles.inputIcon} />
                  <TextInput
                    style={styles.input}
                    placeholder="Код подтверждения"
                    placeholderTextColor="#999"
                    value={verificationCode}
                    onChangeText={setVerificationCode}
                    keyboardType="number-pad"
                    maxLength={6}
                  />
                </View>

                <TouchableOpacity
                  style={[styles.button, isLoading && styles.buttonDisabled]}
                  onPress={handleVerify}
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <ActivityIndicator color="#fff" />
                  ) : (
                    <>
                      <Icon name="check-circle" size={24} color="#fff" />
                      <Text style={styles.buttonText}>Подтвердить</Text>
                    </>
                  )}
                </TouchableOpacity>

                <TouchableOpacity
                  style={styles.backButton}
                  onPress={handleBackToRegister}
                >
                  <Icon name="arrow-back" size={20} color="#667eea" />
                  <Text style={styles.backButtonText}>Назад к регистрации</Text>
                </TouchableOpacity>
              </>
            )}
          </View>

          <View style={styles.footer}>
            <Text style={styles.footerText}>
              Используя приложение, вы соглашаетесь с условиями использования
            </Text>
          </View>
        </ScrollView>
      </KeyboardAvoidingView>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  keyboardAvoidingView: {
    flex: 1,
  },
  scrollContainer: {
    flexGrow: 1,
    justifyContent: 'center',
    padding: 20,
  },
  header: {
    alignItems: 'center',
    marginBottom: 40,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#fff',
    marginTop: 20,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: 16,
    color: '#fff',
    opacity: 0.9,
    marginTop: 10,
    textAlign: 'center',
  },
  formContainer: {
    backgroundColor: '#fff',
    borderRadius: 20,
    padding: 30,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 10,
    },
    shadowOpacity: 0.25,
    shadowRadius: 20,
    elevation: 10,
  },
  formTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#333',
    textAlign: 'center',
    marginBottom: 30,
  },
  verificationText: {
    fontSize: 16,
    color: '#666',
    textAlign: 'center',
    marginBottom: 30,
    lineHeight: 22,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#f8f9fa',
    borderRadius: 12,
    marginBottom: 20,
    paddingHorizontal: 15,
    borderWidth: 2,
    borderColor: '#e9ecef',
  },
  inputIcon: {
    marginRight: 10,
  },
  input: {
    flex: 1,
    paddingVertical: 15,
    fontSize: 16,
    color: '#333',
  },
  button: {
    backgroundColor: '#667eea',
    borderRadius: 12,
    paddingVertical: 15,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    marginTop: 10,
  },
  buttonDisabled: {
    opacity: 0.6,
  },
  buttonText: {
    color: '#fff',
    fontSize: 18,
    fontWeight: 'bold',
    marginLeft: 10,
  },
  backButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    marginTop: 20,
  },
  backButtonText: {
    color: '#667eea',
    fontSize: 16,
    marginLeft: 5,
  },
  footer: {
    marginTop: 30,
    alignItems: 'center',
  },
  footerText: {
    color: '#fff',
    fontSize: 12,
    opacity: 0.8,
    textAlign: 'center',
    lineHeight: 18,
  },
});

export default LoginScreen;
