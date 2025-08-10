import React, { useEffect, useState } from 'react';
import {
  SafeAreaView,
  StatusBar,
  StyleSheet,
  useColorScheme,
  Alert,
  AppState,
  AppStateStatus,
} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { createStackNavigator } from '@react-navigation/stack';
import Icon from 'react-native-vector-icons/MaterialIcons';
import Toast from 'react-native-toast-message';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import DeviceInfo from 'react-native-device-info';
import NetInfo from '@react-native-community/netinfo';

// Screens
import ChatScreen from './screens/ChatScreen';
import ProfileScreen from './screens/ProfileScreen';
import SettingsScreen from './screens/SettingsScreen';
import AnalyticsScreen from './screens/AnalyticsScreen';
import LoginScreen from './screens/LoginScreen';
import OnboardingScreen from './screens/OnboardingScreen';
import SplashScreen from './screens/SplashScreen';

// Services
import { AuthService } from './services/AuthService';
import { ConsciousnessService } from './services/ConsciousnessService';
import { NotificationService } from './services/NotificationService';
import { AnalyticsService } from './services/AnalyticsService';

// Store
import { useAuthStore } from './store/authStore';
import { useAppStore } from './store/appStore';

// Types
import { RootStackParamList, MainTabParamList } from './types/navigation';

// Theme
import { lightTheme, darkTheme } from './theme/colors';
import { ThemeProvider } from './context/ThemeContext';

const Stack = createStackNavigator<RootStackParamList>();
const Tab = createBottomTabNavigator<MainTabParamList>();

const MainTabs: React.FC = () => {
  const colorScheme = useColorScheme();
  const theme = colorScheme === 'dark' ? darkTheme : lightTheme;

  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: string;

          switch (route.name) {
            case 'Chat':
              iconName = 'chat';
              break;
            case 'Analytics':
              iconName = 'analytics';
              break;
            case 'Profile':
              iconName = 'person';
              break;
            case 'Settings':
              iconName = 'settings';
              break;
            default:
              iconName = 'help';
          }

          return <Icon name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: theme.primary,
        tabBarInactiveTintColor: theme.textSecondary,
        tabBarStyle: {
          backgroundColor: theme.surface,
          borderTopColor: theme.border,
        },
        headerStyle: {
          backgroundColor: theme.surface,
        },
        headerTintColor: theme.text,
      })}
    >
      <Tab.Screen 
        name="Chat" 
        component={ChatScreen}
        options={{
          title: 'Consciousness Chat',
          headerShown: false,
        }}
      />
      <Tab.Screen 
        name="Analytics" 
        component={AnalyticsScreen}
        options={{
          title: 'Analytics',
        }}
      />
      <Tab.Screen 
        name="Profile" 
        component={ProfileScreen}
        options={{
          title: 'Profile',
        }}
      />
      <Tab.Screen 
        name="Settings" 
        component={SettingsScreen}
        options={{
          title: 'Settings',
        }}
      />
    </Tab.Navigator>
  );
};

const App: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [isFirstLaunch, setIsFirstLaunch] = useState(false);
  const [appState, setAppState] = useState<AppStateStatus>(AppState.currentState);
  
  const { isAuthenticated, user, initializeAuth } = useAuthStore();
  const { 
    isOnline, 
    setIsOnline, 
    deviceInfo, 
    setDeviceInfo,
    initializeApp 
  } = useAppStore();

  const colorScheme = useColorScheme();
  const theme = colorScheme === 'dark' ? darkTheme : lightTheme;

  useEffect(() => {
    initializeApplication();
  }, []);

  useEffect(() => {
    const handleAppStateChange = (nextAppState: AppStateStatus) => {
      if (appState.match(/inactive|background/) && nextAppState === 'active') {
        // App has come to the foreground
        handleAppForeground();
      } else if (nextAppState.match(/inactive|background/)) {
        // App has gone to the background
        handleAppBackground();
      }
      setAppState(nextAppState);
    };

    const subscription = AppState.addEventListener('change', handleAppStateChange);
    return () => subscription?.remove();
  }, [appState]);

  useEffect(() => {
    // Network connectivity monitoring
    const unsubscribe = NetInfo.addEventListener(state => {
      setIsOnline(state.isConnected ?? false);
      
      if (!state.isConnected) {
        Toast.show({
          type: 'error',
          text1: 'Connection Lost',
          text2: 'Please check your internet connection',
        });
      } else if (isOnline === false && state.isConnected) {
        Toast.show({
          type: 'success',
          text1: 'Connection Restored',
          text2: 'You are back online',
        });
      }
    });

    return () => unsubscribe();
  }, [isOnline, setIsOnline]);

  const initializeApplication = async () => {
    try {
      // Initialize device info
      const deviceData = {
        deviceId: await DeviceInfo.getUniqueId(),
        deviceName: await DeviceInfo.getDeviceName(),
        systemName: DeviceInfo.getSystemName(),
        systemVersion: DeviceInfo.getSystemVersion(),
        appVersion: DeviceInfo.getVersion(),
        buildNumber: DeviceInfo.getBuildNumber(),
        bundleId: DeviceInfo.getBundleId(),
        isEmulator: await DeviceInfo.isEmulator(),
        hasNotch: DeviceInfo.hasNotch(),
        brand: DeviceInfo.getBrand(),
        model: DeviceInfo.getModel(),
      };
      setDeviceInfo(deviceData);

      // Check if first launch
      const hasLaunched = await AsyncStorage.getItem('hasLaunched');
      if (!hasLaunched) {
        setIsFirstLaunch(true);
        await AsyncStorage.setItem('hasLaunched', 'true');
      }

      // Initialize services
      await Promise.all([
        initializeAuth(),
        initializeApp(),
        NotificationService.initialize(),
        AnalyticsService.initialize(deviceData),
      ]);

      // Track app launch
      AnalyticsService.trackEvent('app_launched', {
        isFirstLaunch,
        deviceInfo: deviceData,
      });

    } catch (error) {
      console.error('Failed to initialize app:', error);
      Alert.alert(
        'Initialization Error',
        'Failed to initialize the app. Please restart the application.',
        [{ text: 'OK' }]
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleAppForeground = async () => {
    try {
      // Refresh authentication if needed
      if (isAuthenticated) {
        await AuthService.refreshTokenIfNeeded();
      }

      // Check for app updates
      // await checkForUpdates();

      // Sync offline data
      if (isOnline) {
        await ConsciousnessService.syncOfflineData();
      }

      AnalyticsService.trackEvent('app_foreground');
    } catch (error) {
      console.error('Error handling app foreground:', error);
    }
  };

  const handleAppBackground = () => {
    try {
      // Save app state
      // Clear sensitive data from memory if needed
      
      AnalyticsService.trackEvent('app_background');
    } catch (error) {
      console.error('Error handling app background:', error);
    }
  };

  if (isLoading) {
    return <SplashScreen />;
  }

  return (
    <GestureHandlerRootView style={styles.container}>
      <ThemeProvider>
        <SafeAreaView style={[styles.container, { backgroundColor: theme.background }]}>
          <StatusBar
            barStyle={colorScheme === 'dark' ? 'light-content' : 'dark-content'}
            backgroundColor={theme.surface}
          />
          
          <NavigationContainer
            theme={{
              dark: colorScheme === 'dark',
              colors: {
                primary: theme.primary,
                background: theme.background,
                card: theme.surface,
                text: theme.text,
                border: theme.border,
                notification: theme.accent,
              },
            }}
          >
            <Stack.Navigator
              screenOptions={{
                headerShown: false,
                gestureEnabled: true,
                cardStyleInterpolator: ({ current, layouts }) => ({
                  cardStyle: {
                    transform: [
                      {
                        translateX: current.progress.interpolate({
                          inputRange: [0, 1],
                          outputRange: [layouts.screen.width, 0],
                        }),
                      },
                    ],
                  },
                }),
              }}
            >
              {isFirstLaunch ? (
                <Stack.Screen name="Onboarding" component={OnboardingScreen} />
              ) : !isAuthenticated ? (
                <Stack.Screen name="Login" component={LoginScreen} />
              ) : (
                <Stack.Screen name="Main" component={MainTabs} />
              )}
            </Stack.Navigator>
          </NavigationContainer>

          <Toast />
        </SafeAreaView>
      </ThemeProvider>
    </GestureHandlerRootView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});

export default App;
