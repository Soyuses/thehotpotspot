// Utility functions for formatting data

export const formatCurrency = (amount: number, currency: string = 'GEL'): string => {
  return new Intl.NumberFormat('ka-GE', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
};

export const formatSubunits = (subunits: number, currency: string = 'GEL'): string => {
  const amount = subunits / 100;
  return formatCurrency(amount, currency);
};

export const formatTokens = (tokens: number, tokenType: 'ST' | 'UT' = 'ST'): string => {
  const symbol = tokenType === 'ST' ? 'THP' : 'SPOT';
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(tokens) + ` ${symbol}`;
};

export const formatDate = (date: string | Date): string => {
  const d = typeof date === 'string' ? new Date(date) : date;
  return new Intl.DateTimeFormat('ka-GE', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(d);
};

export const formatRelativeTime = (date: string | Date): string => {
  const d = typeof date === 'string' ? new Date(date) : date;
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - d.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return 'только что';
  } else if (diffInSeconds < 3600) {
    const minutes = Math.floor(diffInSeconds / 60);
    return `${minutes} мин. назад`;
  } else if (diffInSeconds < 86400) {
    const hours = Math.floor(diffInSeconds / 3600);
    return `${hours} ч. назад`;
  } else {
    const days = Math.floor(diffInSeconds / 86400);
    return `${days} дн. назад`;
  }
};

export const formatDuration = (minutes: number): string => {
  if (minutes < 60) {
    return `${minutes} мин.`;
  } else {
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    return remainingMinutes > 0 
      ? `${hours} ч. ${remainingMinutes} мин.`
      : `${hours} ч.`;
  }
};

export const formatPercentage = (value: number, decimals: number = 1): string => {
  return new Intl.NumberFormat('en-US', {
    style: 'percent',
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value / 100);
};

export const formatNumber = (value: number, decimals: number = 0): string => {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
};

export const truncateAddress = (address: string, start: number = 6, end: number = 4): string => {
  if (address.length <= start + end) {
    return address;
  }
  return `${address.slice(0, start)}...${address.slice(-end)}`;
};

export const capitalizeFirst = (str: string): string => {
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

export const formatOrderStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    'pending': 'Ожидает',
    'confirmed': 'Подтвержден',
    'cooking': 'Готовится',
    'ready': 'Готов',
    'delivered': 'Доставлен',
    'cancelled': 'Отменен'
  };
  return statusMap[status] || capitalizeFirst(status);
};

export const formatKYCStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    'pending': 'Ожидает',
    'verified': 'Верифицирован',
    'rejected': 'Отклонен',
    'expired': 'Истек'
  };
  return statusMap[status] || capitalizeFirst(status);
};

export const formatKYCLevel = (level: string): string => {
  const levelMap: Record<string, string> = {
    'basic': 'Базовый',
    'enhanced': 'Расширенный',
    'premium': 'Премиум'
  };
  return levelMap[level] || capitalizeFirst(level);
};
