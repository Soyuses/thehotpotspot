import React, { useState, useEffect } from 'react';
import {
  View,
  StyleSheet,
  Alert,
  Dimensions,
} from 'react-native';
import {
  Text,
  Button,
  Card,
  Title,
  Paragraph,
  ActivityIndicator,
} from 'react-native-paper';
import { RNCamera } from 'react-native-camera';
import { useWallet } from '../contexts/WalletContext';
import { useTheme } from '../contexts/ThemeContext';

const { width, height } = Dimensions.get('window');

const QRScannerScreen: React.FC = () => {
  const [isScanning, setIsScanning] = useState(true);
  const [hasPermission, setHasPermission] = useState<boolean | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const { claimCheck } = useWallet();
  const { theme } = useTheme();

  useEffect(() => {
    // Request camera permission
    requestCameraPermission();
  }, []);

  const requestCameraPermission = async () => {
    try {
      const permission = await RNCamera.requestCameraPermission();
      setHasPermission(permission === 'authorized');
    } catch (error) {
      console.error('Error requesting camera permission:', error);
      setHasPermission(false);
    }
  };

  const handleQRCodeRead = async (event: any) => {
    if (isProcessing) return;

    setIsScanning(false);
    setIsProcessing(true);

    try {
      const qrData = event.data;
      console.log('QR Code scanned:', qrData);

      // Validate QR code format
      if (!qrData || !qrData.includes('thehotpotspot')) {
        Alert.alert(
          'Неверный QR-код',
          'Этот QR-код не относится к The Hot Pot Spot',
          [
            {
              text: 'Попробовать снова',
              onPress: () => {
                setIsScanning(true);
                setIsProcessing(false);
              },
            },
          ]
        );
        return;
      }

      // Claim the check
      const success = await claimCheck(qrData);

      if (success) {
        Alert.alert(
          'Успешно!',
          'Чек успешно активирован. Токены добавлены в ваш кошелек.',
          [
            {
              text: 'OK',
              onPress: () => {
                // Navigate back to wallet screen
                console.log('Navigate to wallet');
              },
            },
          ]
        );
      } else {
        Alert.alert(
          'Ошибка',
          'Не удалось активировать чек. Возможно, он уже был использован.',
          [
            {
              text: 'Попробовать снова',
              onPress: () => {
                setIsScanning(true);
                setIsProcessing(false);
              },
            },
          ]
        );
      }
    } catch (error) {
      console.error('Error processing QR code:', error);
      Alert.alert(
        'Ошибка',
        'Произошла ошибка при обработке QR-кода',
        [
          {
            text: 'Попробовать снова',
            onPress: () => {
              setIsScanning(true);
              setIsProcessing(false);
            },
          },
        ]
      );
    }
  };

  if (hasPermission === null) {
    return (
      <View style={styles.centerContainer}>
        <ActivityIndicator size="large" color={theme.colors.primary} />
        <Text style={[styles.loadingText, { color: theme.colors.onSurface }]}>
          Запрос разрешения на камеру...
        </Text>
      </View>
    );
  }

  if (hasPermission === false) {
    return (
      <View style={styles.centerContainer}>
        <Card style={[styles.card, { backgroundColor: theme.colors.surface }]}>
          <Card.Content style={styles.cardContent}>
            <Title style={[styles.title, { color: theme.colors.primary }]}>
              Доступ к камере запрещен
            </Title>
            <Paragraph style={[styles.paragraph, { color: theme.colors.onSurface }]}>
              Для сканирования QR-кодов необходимо разрешение на использование камеры.
              Пожалуйста, разрешите доступ к камере в настройках приложения.
            </Paragraph>
            <Button
              mode="contained"
              onPress={requestCameraPermission}
              style={[styles.button, { backgroundColor: theme.colors.primary }]}
            >
              Попробовать снова
            </Button>
          </Card.Content>
        </Card>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Title style={[styles.headerTitle, { color: theme.colors.primary }]}>
          Сканирование QR-кода
        </Title>
        <Paragraph style={[styles.headerSubtitle, { color: theme.colors.onSurface }]}>
          Наведите камеру на QR-код с чека
        </Paragraph>
      </View>

      <View style={styles.cameraContainer}>
        <RNCamera
          style={styles.camera}
          type={RNCamera.Constants.Type.back}
          flashMode={RNCamera.Constants.FlashMode.auto}
          onBarCodeRead={isScanning ? handleQRCodeRead : undefined}
          barCodeTypes={[RNCamera.Constants.BarCodeType.qr]}
        >
          <View style={styles.overlay}>
            <View style={styles.scanArea}>
              <View style={[styles.corner, styles.topLeft]} />
              <View style={[styles.corner, styles.topRight]} />
              <View style={[styles.corner, styles.bottomLeft]} />
              <View style={[styles.corner, styles.bottomRight]} />
            </View>
          </View>
        </RNCamera>
      </View>

      {isProcessing && (
        <View style={styles.processingOverlay}>
          <Card style={[styles.processingCard, { backgroundColor: theme.colors.surface }]}>
            <Card.Content style={styles.processingContent}>
              <ActivityIndicator size="large" color={theme.colors.primary} />
              <Text style={[styles.processingText, { color: theme.colors.onSurface }]}>
                Обработка QR-кода...
              </Text>
            </Card.Content>
          </Card>
        </View>
      )}

      <View style={styles.footer}>
        <Button
          mode="outlined"
          onPress={() => {
            // Navigate back
            console.log('Navigate back');
          }}
          style={styles.backButton}
        >
          Назад
        </Button>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000',
  },
  centerContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
    backgroundColor: '#f5f5f5',
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
  },
  card: {
    elevation: 4,
    borderRadius: 12,
    width: '100%',
  },
  cardContent: {
    padding: 20,
    alignItems: 'center',
  },
  title: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 12,
    textAlign: 'center',
  },
  paragraph: {
    fontSize: 14,
    textAlign: 'center',
    marginBottom: 20,
    lineHeight: 20,
  },
  button: {
    borderRadius: 8,
  },
  header: {
    position: 'absolute',
    top: 50,
    left: 0,
    right: 0,
    zIndex: 10,
    paddingHorizontal: 20,
    alignItems: 'center',
  },
  headerTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  headerSubtitle: {
    fontSize: 14,
    textAlign: 'center',
  },
  cameraContainer: {
    flex: 1,
  },
  camera: {
    flex: 1,
  },
  overlay: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  scanArea: {
    width: width * 0.7,
    height: width * 0.7,
    position: 'relative',
  },
  corner: {
    position: 'absolute',
    width: 30,
    height: 30,
    borderColor: '#fff',
    borderWidth: 3,
  },
  topLeft: {
    top: 0,
    left: 0,
    borderRightWidth: 0,
    borderBottomWidth: 0,
  },
  topRight: {
    top: 0,
    right: 0,
    borderLeftWidth: 0,
    borderBottomWidth: 0,
  },
  bottomLeft: {
    bottom: 0,
    left: 0,
    borderRightWidth: 0,
    borderTopWidth: 0,
  },
  bottomRight: {
    bottom: 0,
    right: 0,
    borderLeftWidth: 0,
    borderTopWidth: 0,
  },
  processingOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: 'rgba(0, 0, 0, 0.7)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  processingCard: {
    elevation: 8,
    borderRadius: 12,
    width: '80%',
  },
  processingContent: {
    padding: 20,
    alignItems: 'center',
  },
  processingText: {
    marginTop: 16,
    fontSize: 16,
    textAlign: 'center',
  },
  footer: {
    position: 'absolute',
    bottom: 50,
    left: 0,
    right: 0,
    paddingHorizontal: 20,
  },
  backButton: {
    borderRadius: 8,
  },
});

export default QRScannerScreen;

