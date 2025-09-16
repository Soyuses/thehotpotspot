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
  ProgressBar,
  Chip,
  ActivityIndicator,
  Divider,
} from 'react-native-paper';
import { votingAPI } from '../services/api';
import { useTheme } from '../contexts/ThemeContext';

interface Proposal {
  id: string;
  title: string;
  description: string;
  type: 'tokenomics_change' | 'treasury_allocation' | 'protocol_upgrade' | 'emergency_action' | 'general';
  status: 'active' | 'passed' | 'rejected' | 'executed';
  creator: string;
  createdAt: string;
  votingEndsAt: string;
  totalVotes: number;
  yesVotes: number;
  noVotes: number;
  abstainVotes: number;
  userVote?: 'yes' | 'no' | 'abstain';
  userVotingPower: number;
  executionData?: any;
}

const VotingScreen: React.FC = () => {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [selectedTab, setSelectedTab] = useState<'active' | 'all'>('active');
  const { theme } = useTheme();

  useEffect(() => {
    loadProposals();
  }, []);

  const loadProposals = async () => {
    try {
      setIsLoading(true);
      const response = await votingAPI.getProposals();
      
      if (response.success && response.data) {
        setProposals(response.data);
      } else {
        // Mock data for demo
        setProposals(getMockProposals());
      }
    } catch (error) {
      console.error('Error loading proposals:', error);
      setProposals(getMockProposals());
    } finally {
      setIsLoading(false);
    }
  };

  const getMockProposals = (): Proposal[] => [
    {
      id: '1',
      title: 'Увеличение коэффициента эмиссии ST токенов',
      description: 'Предлагается увеличить коэффициент эмиссии Security токенов с 0.2 до 0.25 THP за 1 GEL для повышения привлекательности для инвесторов.',
      type: 'tokenomics_change',
      status: 'active',
      creator: '0x1234...5678',
      createdAt: '2024-01-15T10:00:00Z',
      votingEndsAt: '2024-01-22T10:00:00Z',
      totalVotes: 1250,
      yesVotes: 750,
      noVotes: 400,
      abstainVotes: 100,
      userVote: 'yes',
      userVotingPower: 150,
    },
    {
      id: '2',
      title: 'Выделение средств на маркетинг',
      description: 'Предлагается выделить 10,000 GEL из казны DAO на маркетинговую кампанию для привлечения новых пользователей.',
      type: 'treasury_allocation',
      status: 'active',
      creator: '0x2345...6789',
      createdAt: '2024-01-14T15:30:00Z',
      votingEndsAt: '2024-01-21T15:30:00Z',
      totalVotes: 980,
      yesVotes: 520,
      noVotes: 380,
      abstainVotes: 80,
      userVote: undefined,
      userVotingPower: 150,
    },
    {
      id: '3',
      title: 'Обновление протокола безопасности',
      description: 'Внедрение новых мер безопасности для защиты пользовательских данных и предотвращения мошенничества.',
      type: 'protocol_upgrade',
      status: 'passed',
      creator: '0x3456...7890',
      createdAt: '2024-01-10T09:00:00Z',
      votingEndsAt: '2024-01-17T09:00:00Z',
      totalVotes: 2100,
      yesVotes: 1400,
      noVotes: 600,
      abstainVotes: 100,
      userVote: 'yes',
      userVotingPower: 150,
    },
  ];

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadProposals();
    setRefreshing(false);
  };

  const handleVote = async (proposalId: string, choice: 'yes' | 'no' | 'abstain') => {
    try {
      const response = await votingAPI.castVote(proposalId, choice);
      
      if (response.success) {
        Alert.alert('Успешно!', 'Ваш голос засчитан');
        
        // Update local state
        setProposals(prev => prev.map(proposal => 
          proposal.id === proposalId 
            ? { ...proposal, userVote: choice }
            : proposal
        ));
      } else {
        Alert.alert('Ошибка', response.error || 'Не удалось проголосовать');
      }
    } catch (error) {
      Alert.alert('Ошибка', 'Произошла ошибка при голосовании');
    }
  };

  const getProposalTypeLabel = (type: string) => {
    switch (type) {
      case 'tokenomics_change':
        return 'Изменение токеномики';
      case 'treasury_allocation':
        return 'Распределение казны';
      case 'protocol_upgrade':
        return 'Обновление протокола';
      case 'emergency_action':
        return 'Экстренные меры';
      case 'general':
        return 'Общие вопросы';
      default:
        return 'Неизвестно';
    }
  };

  const getProposalTypeColor = (type: string) => {
    switch (type) {
      case 'tokenomics_change':
        return theme.colors.primary;
      case 'treasury_allocation':
        return theme.colors.secondary;
      case 'protocol_upgrade':
        return theme.colors.tertiary;
      case 'emergency_action':
        return theme.colors.error;
      case 'general':
        return theme.colors.onSurface;
      default:
        return theme.colors.onSurface;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return theme.colors.primary;
      case 'passed':
        return '#4CAF50';
      case 'rejected':
        return theme.colors.error;
      case 'executed':
        return '#2196F3';
      default:
        return theme.colors.onSurface;
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'active':
        return 'Активно';
      case 'passed':
        return 'Принято';
      case 'rejected':
        return 'Отклонено';
      case 'executed':
        return 'Выполнено';
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
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const calculateVotingProgress = (proposal: Proposal) => {
    if (proposal.totalVotes === 0) return 0;
    return proposal.yesVotes / proposal.totalVotes;
  };

  const getTimeRemaining = (endDate: string) => {
    const now = new Date();
    const end = new Date(endDate);
    const diff = end.getTime() - now.getTime();
    
    if (diff <= 0) return 'Завершено';
    
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    
    if (days > 0) return `${days} дн. ${hours} ч.`;
    return `${hours} ч.`;
  };

  const filteredProposals = selectedTab === 'active' 
    ? proposals.filter(p => p.status === 'active')
    : proposals;

  const renderProposalCard = (proposal: Proposal) => (
    <Card key={proposal.id} style={[styles.proposalCard, { backgroundColor: theme.colors.surface }]}>
      <Card.Content style={styles.proposalContent}>
        <View style={styles.proposalHeader}>
          <View style={styles.proposalTitleContainer}>
            <Title style={[styles.proposalTitle, { color: theme.colors.onSurface }]}>
              {proposal.title}
            </Title>
            <View style={styles.proposalMeta}>
              <Chip
                style={[styles.typeChip, { backgroundColor: getProposalTypeColor(proposal.type) }]}
                textStyle={{ color: theme.colors.onPrimary }}
              >
                {getProposalTypeLabel(proposal.type)}
              </Chip>
              <Chip
                style={[styles.statusChip, { backgroundColor: getStatusColor(proposal.status) }]}
                textStyle={{ color: theme.colors.onPrimary }}
              >
                {getStatusLabel(proposal.status)}
              </Chip>
            </View>
          </View>
        </View>

        <Paragraph style={[styles.proposalDescription, { color: theme.colors.onSurface }]}>
          {proposal.description}
        </Paragraph>

        <View style={styles.proposalStats}>
          <View style={styles.statItem}>
            <Text style={[styles.statLabel, { color: theme.colors.onSurface }]}>
              Всего голосов
            </Text>
            <Text style={[styles.statValue, { color: theme.colors.primary }]}>
              {proposal.totalVotes.toLocaleString()}
            </Text>
          </View>
          <View style={styles.statItem}>
            <Text style={[styles.statLabel, { color: theme.colors.onSurface }]}>
              Ваша сила голоса
            </Text>
            <Text style={[styles.statValue, { color: theme.colors.secondary }]}>
              {proposal.userVotingPower}
            </Text>
          </View>
        </View>

        {proposal.status === 'active' && (
          <View style={styles.votingSection}>
            <View style={styles.votingProgress}>
              <View style={styles.progressHeader}>
                <Text style={[styles.progressLabel, { color: theme.colors.onSurface }]}>
                  Прогресс голосования
                </Text>
                <Text style={[styles.progressValue, { color: theme.colors.onSurface }]}>
                  {Math.round(calculateVotingProgress(proposal) * 100)}%
                </Text>
              </View>
              <ProgressBar
                progress={calculateVotingProgress(proposal)}
                color={theme.colors.primary}
                style={styles.progressBar}
              />
              <View style={styles.voteCounts}>
                <Text style={[styles.voteCount, { color: '#4CAF50' }]}>
                  За: {proposal.yesVotes}
                </Text>
                <Text style={[styles.voteCount, { color: theme.colors.error }]}>
                  Против: {proposal.noVotes}
                </Text>
                <Text style={[styles.voteCount, { color: theme.colors.onSurface }]}>
                  Воздержались: {proposal.abstainVotes}
                </Text>
              </View>
            </View>

            <View style={styles.votingActions}>
              <Text style={[styles.timeRemaining, { color: theme.colors.onSurface }]}>
                Осталось: {getTimeRemaining(proposal.votingEndsAt)}
              </Text>
              
              {proposal.userVote ? (
                <View style={styles.userVoteContainer}>
                  <Text style={[styles.userVoteText, { color: theme.colors.primary }]}>
                    Ваш голос: {proposal.userVote === 'yes' ? 'За' : proposal.userVote === 'no' ? 'Против' : 'Воздержался'}
                  </Text>
                </View>
              ) : (
                <View style={styles.voteButtons}>
                  <Button
                    mode="contained"
                    onPress={() => handleVote(proposal.id, 'yes')}
                    style={[styles.voteButton, { backgroundColor: '#4CAF50' }]}
                    compact
                  >
                    За
                  </Button>
                  <Button
                    mode="contained"
                    onPress={() => handleVote(proposal.id, 'no')}
                    style={[styles.voteButton, { backgroundColor: theme.colors.error }]}
                    compact
                  >
                    Против
                  </Button>
                  <Button
                    mode="outlined"
                    onPress={() => handleVote(proposal.id, 'abstain')}
                    style={styles.voteButton}
                    compact
                  >
                    Воздержаться
                  </Button>
                </View>
              )}
            </View>
          </View>
        )}

        <Divider style={styles.divider} />
        
        <View style={styles.proposalFooter}>
          <Text style={[styles.footerText, { color: theme.colors.onSurface }]}>
            Создатель: {proposal.creator}
          </Text>
          <Text style={[styles.footerText, { color: theme.colors.onSurface }]}>
            {formatDate(proposal.createdAt)}
          </Text>
        </View>
      </Card.Content>
    </Card>
  );

  if (isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={theme.colors.primary} />
        <Text style={[styles.loadingText, { color: theme.colors.onSurface }]}>
          Загрузка предложений...
        </Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Title style={[styles.headerTitle, { color: theme.colors.primary }]}>
          Голосование DAO
        </Title>
        <View style={styles.tabs}>
          <Button
            mode={selectedTab === 'active' ? 'contained' : 'outlined'}
            onPress={() => setSelectedTab('active')}
            style={styles.tabButton}
          >
            Активные
          </Button>
          <Button
            mode={selectedTab === 'all' ? 'contained' : 'outlined'}
            onPress={() => setSelectedTab('all')}
            style={styles.tabButton}
          >
            Все
          </Button>
        </View>
      </View>

      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={handleRefresh} />
        }
      >
        <View style={styles.proposalsContainer}>
          {filteredProposals.length === 0 ? (
            <View style={styles.emptyContainer}>
              <Text style={[styles.emptyText, { color: theme.colors.onSurface }]}>
                Предложения не найдены
              </Text>
              <Paragraph style={[styles.emptySubtext, { color: theme.colors.onSurface }]}>
                {selectedTab === 'active' 
                  ? 'В данный момент нет активных предложений для голосования'
                  : 'Предложения появятся здесь после их создания'
                }
              </Paragraph>
            </View>
          ) : (
            filteredProposals.map(renderProposalCard)
          )}
        </View>
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
  header: {
    padding: 16,
    backgroundColor: '#fff',
    elevation: 2,
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 16,
  },
  tabs: {
    flexDirection: 'row',
    gap: 8,
  },
  tabButton: {
    flex: 1,
    borderRadius: 8,
  },
  scrollView: {
    flex: 1,
  },
  proposalsContainer: {
    padding: 16,
  },
  proposalCard: {
    marginBottom: 16,
    elevation: 2,
    borderRadius: 12,
  },
  proposalContent: {
    padding: 16,
  },
  proposalHeader: {
    marginBottom: 12,
  },
  proposalTitleContainer: {
    marginBottom: 8,
  },
  proposalTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  proposalMeta: {
    flexDirection: 'row',
    gap: 8,
  },
  typeChip: {
    borderRadius: 16,
  },
  statusChip: {
    borderRadius: 16,
  },
  proposalDescription: {
    fontSize: 14,
    lineHeight: 20,
    marginBottom: 16,
  },
  proposalStats: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginBottom: 16,
    padding: 12,
    backgroundColor: '#f8f9fa',
    borderRadius: 8,
  },
  statItem: {
    alignItems: 'center',
  },
  statLabel: {
    fontSize: 12,
    marginBottom: 4,
  },
  statValue: {
    fontSize: 16,
    fontWeight: 'bold',
  },
  votingSection: {
    marginBottom: 16,
  },
  votingProgress: {
    marginBottom: 16,
  },
  progressHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 8,
  },
  progressLabel: {
    fontSize: 14,
    fontWeight: 'bold',
  },
  progressValue: {
    fontSize: 14,
    fontWeight: 'bold',
  },
  progressBar: {
    height: 8,
    borderRadius: 4,
    marginBottom: 8,
  },
  voteCounts: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  voteCount: {
    fontSize: 12,
    fontWeight: 'bold',
  },
  votingActions: {
    alignItems: 'center',
  },
  timeRemaining: {
    fontSize: 14,
    fontWeight: 'bold',
    marginBottom: 12,
  },
  userVoteContainer: {
    padding: 12,
    backgroundColor: '#e3f2fd',
    borderRadius: 8,
    width: '100%',
    alignItems: 'center',
  },
  userVoteText: {
    fontSize: 14,
    fontWeight: 'bold',
  },
  voteButtons: {
    flexDirection: 'row',
    gap: 8,
    width: '100%',
  },
  voteButton: {
    flex: 1,
    borderRadius: 8,
  },
  divider: {
    marginVertical: 12,
  },
  proposalFooter: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  footerText: {
    fontSize: 12,
    opacity: 0.7,
  },
  emptyContainer: {
    alignItems: 'center',
    padding: 40,
  },
  emptyText: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  emptySubtext: {
    fontSize: 14,
    textAlign: 'center',
    lineHeight: 20,
  },
});

export default VotingScreen;

