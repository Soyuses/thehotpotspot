// Validation utilities

export const isValidEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

export const isValidPhone = (phone: string): boolean => {
  const phoneRegex = /^\+?[1-9]\d{1,14}$/;
  return phoneRegex.test(phone.replace(/\s/g, ''));
};

export const isValidWalletAddress = (address: string): boolean => {
  // Basic validation for wallet addresses (starts with 0x and has correct length)
  return /^0x[a-fA-F0-9]{40}$/.test(address);
};

export const isValidAmount = (amount: number): boolean => {
  return amount > 0 && amount < Number.MAX_SAFE_INTEGER;
};

export const isValidQuantity = (quantity: number): boolean => {
  return Number.isInteger(quantity) && quantity > 0 && quantity <= 1000;
};

export const isValidPassword = (password: string): boolean => {
  // At least 8 characters, 1 uppercase, 1 lowercase, 1 number
  const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d@$!%*?&]{8,}$/;
  return passwordRegex.test(password);
};

export const isValidName = (name: string): boolean => {
  return name.length >= 2 && name.length <= 50 && /^[a-zA-Zа-яА-Я\s-']+$/.test(name);
};

export const isValidPostalCode = (code: string): boolean => {
  // Georgian postal code format (4 digits)
  return /^\d{4}$/.test(code);
};

export const isValidDateOfBirth = (date: string): boolean => {
  const birthDate = new Date(date);
  const today = new Date();
  const age = today.getFullYear() - birthDate.getFullYear();
  
  return age >= 18 && age <= 120 && birthDate < today;
};

export const isValidStreamName = (name: string): boolean => {
  return name.length >= 3 && name.length <= 100;
};

export const isValidExchangeRate = (rate: number): boolean => {
  return Number.isInteger(rate) && rate >= 1 && rate <= 1000;
};

export const validateOrderItem = (item: { menu_item_id: string; quantity: number; price_subunits: number }): string[] => {
  const errors: string[] = [];
  
  if (!item.menu_item_id) {
    errors.push('ID блюда обязателен');
  }
  
  if (!isValidQuantity(item.quantity)) {
    errors.push('Количество должно быть от 1 до 1000');
  }
  
  if (!isValidAmount(item.price_subunits)) {
    errors.push('Цена должна быть положительной');
  }
  
  return errors;
};

export const validateCreateOrder = (order: { customer_wallet: string; items: any[]; delivery_time_minutes: number }): string[] => {
  const errors: string[] = [];
  
  if (!isValidWalletAddress(order.customer_wallet)) {
    errors.push('Неверный адрес кошелька');
  }
  
  if (!order.items || order.items.length === 0) {
    errors.push('Заказ должен содержать хотя бы одно блюдо');
  } else {
    order.items.forEach((item, index) => {
      const itemErrors = validateOrderItem(item);
      itemErrors.forEach(error => errors.push(`Блюдо ${index + 1}: ${error}`));
    });
  }
  
  if (!Number.isInteger(order.delivery_time_minutes) || order.delivery_time_minutes < 1 || order.delivery_time_minutes > 300) {
    errors.push('Время доставки должно быть от 1 до 300 минут');
  }
  
  return errors;
};

export const validateUserRegistration = (user: { email: string; first_name: string; last_name: string; phone?: string }): string[] => {
  const errors: string[] = [];
  
  if (!isValidEmail(user.email)) {
    errors.push('Неверный формат email');
  }
  
  if (!isValidName(user.first_name)) {
    errors.push('Имя должно содержать только буквы и быть от 2 до 50 символов');
  }
  
  if (!isValidName(user.last_name)) {
    errors.push('Фамилия должна содержать только буквы и быть от 2 до 50 символов');
  }
  
  if (user.phone && !isValidPhone(user.phone)) {
    errors.push('Неверный формат телефона');
  }
  
  return errors;
};

export const validateAddress = (address: { street: string; city: string; state: string; country: string; postal_code: string }): string[] => {
  const errors: string[] = [];
  
  if (!address.street || address.street.length < 5) {
    errors.push('Улица должна содержать минимум 5 символов');
  }
  
  if (!address.city || address.city.length < 2) {
    errors.push('Город должен содержать минимум 2 символа');
  }
  
  if (!address.state || address.state.length < 2) {
    errors.push('Регион должен содержать минимум 2 символа');
  }
  
  if (!address.country || address.country.length < 2) {
    errors.push('Страна должна содержать минимум 2 символа');
  }
  
  if (!isValidPostalCode(address.postal_code)) {
    errors.push('Почтовый индекс должен содержать 4 цифры');
  }
  
  return errors;
};

export const validateConversionRequest = (request: { holders: string[]; exchange_rate: number }): string[] => {
  const errors: string[] = [];
  
  if (!request.holders || request.holders.length === 0) {
    errors.push('Выберите хотя бы одного держателя UT');
  }
  
  if (!isValidExchangeRate(request.exchange_rate)) {
    errors.push('Курс обмена должен быть от 1 до 1000');
  }
  
  return errors;
};
