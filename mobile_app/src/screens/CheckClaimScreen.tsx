import React, { useState } from 'react';
import {
  View,
  StyleSheet,
  Alert,
} from 'react-native';
import {
  Text,
  Card,
  Title,
  Paragraph,
  Button,
  ActivityIndicator,
} from 'react-native-paper';
import { useWallet } from '../contexts/WalletContext';
import { useTheme } from '../contexts/ThemeContext';

interface CheckClaimScreenProps {
  qrData: string;
  onSuccess: () => void;
  onCancel: () => void;
}

const CheckClaimScreen: React.FC<CheckClaimScreenProps> = ({
  qrData,
  onSuccess,
  onCancel,
}) => {
  const [isProcessing, setIsProcessing] = useState(false);
  const { claimCheck } = useWallet();
  const { theme } = useTheme();

  const handleClaimCheck = async () => {
    try {
      setIsProcessing(true);
      const success = await claimCheck(qrData);
      
      if (success) {
        onSuccess();
      }
    } catch (error) {
      console.error('Error claiming check:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при активации чека');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleCancel = () => {
    Alert.alert(
      'Отмена',
      'Вы уверены, что хотите отменить активацию чека?',
      [
        { text: 'Продолжить', style: 'cancel' },
        {
          text: 'Отменить',
          style: 'destructive',
          onPress: onCancel,
        },
      ]
    );
  };

  return (
    <View style={styles.container}>
      <Card style={[styles.card, { backgroundColor: theme.colors.surface }]}>
        <Card.Content style={styles.cardContent}>
          <Title style={[styles.title, { color: theme.colors.primary }]}>
            Активация чека
          </Title>
          
          <Paragraph style={[styles.description, { color: theme.colors.onSurface }]}>
            Вы собираетесь активировать чек и получить Security токены на ваш кошелек.
          </Paragraph>

          <View style={styles.qrDataContainer}>
            <Text style={[styles.qrDataLabel, { color: theme.colors.onSurface }]}>
              QR-код:
            </Text>
            <Text style={[styles.qrDataText, { color: theme.colors.onSurface }]}>
              {qrData}
            </Text>
          </View>

          <View style={styles.infoContainer}>
            <Text style={[styles.infoTitle, { color: theme.colors.onSurface }]}>
              Что произойдет:
            </Text>
            <Text style={[styles.infoText, { color: theme.colors.onSurface }]}>
              • Токены будут переведены с анонимного кошелька на ваш личный кошелек
            </Text>
            <Text style={[styles.infoText, { color: theme.colors.onSurface }]}>
              • Вы сможете использовать токены для голосования в DAO
            </Text>
            <Text style={[styles.infoText, { color: theme.colors.onSurface }]}>
              • Токены будут участвовать в распределении дивидендов
            </Text>
          </View>

          <View style={styles.actionsContainer}>
            <Button
              mode="contained"
              onPress={handleClaimCheck}
              style={[styles.claimButton, { backgroundColor: theme.colors.primary }]}
              disabled={isProcessing}
            >
              {isProcessing ? (
                <ActivityIndicator color={theme.colors.onPrimary} size="small" />
              ) : (
                'Активировать чек'
              )}
            </Button>

            <Button
              mode="outlined"
              onPress={handleCancel}
              style={styles.cancelButton}
              disabled={isProcessing}
            >
              Отмена
            </Button>
          </View>
        </Card.Content>
      </Card>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
    backgroundColor: '#f5f5f5',
  },
  card: {
    width: '100%',
    maxWidth: 400,
    elevation: 4,
    borderRadius: 12,
  },
  cardContent: {
    padding: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 16,
    textAlign: 'center',
  },
  description: {
    fontSize: 16,
    lineHeight: 24,
    marginBottom: 20,
    textAlign: 'center',
  },
  qrDataContainer: {
    marginBottom: 20,
    padding: 12,
    backgroundColor: '#f8f9fa',
    borderRadius: 8,
  },
  qrDataLabel: {
    fontSize: 14,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  qrDataText: {
    fontSize: 12,
    fontFamily: 'monospace',
    opacity: 0.8,
  },
  infoContainer: {
    marginBottom: 24,
  },
  infoTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  infoText: {
    fontSize: 14,
    lineHeight: 20,
    marginBottom: 4,
  },
  actionsContainer: {
    gap: 12,
  },
  claimButton: {
    borderRadius: 8,
  },
  cancelButton: {
    borderRadius: 8,
  },
});

export default CheckClaimScreen;

