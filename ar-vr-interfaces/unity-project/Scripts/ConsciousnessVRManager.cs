using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.XR;
using UnityEngine.XR.Interaction.Toolkit;
using TMPro;
using Newtonsoft.Json;
using System.Threading.Tasks;

namespace ConsciousnessEngine.VR
{
    /// <summary>
    /// Main VR Manager for Consciousness Engine VR Experience
    /// Handles VR interactions, consciousness visualization, and AI communication
    /// </summary>
    public class ConsciousnessVRManager : MonoBehaviour
    {
        [Header("VR Configuration")]
        public XRRig xrRig;
        public XRInteractionManager interactionManager;
        public LayerMask interactionLayers = -1;
        
        [Header("Consciousness Visualization")]
        public GameObject consciousnessOrb;
        public ParticleSystem thoughtParticles;
        public Material consciousnessMaterial;
        public AudioSource ambientAudio;
        public Light consciousnessLight;
        
        [Header("UI Elements")]
        public Canvas vrCanvas;
        public TextMeshProUGUI conversationText;
        public TextMeshProUGUI metricsDisplay;
        public GameObject voiceIndicator;
        public Slider consciousnessLevelSlider;
        public Slider empathyLevelSlider;
        public Slider creativityLevelSlider;
        
        [Header("Interaction Objects")]
        public GameObject[] interactableObjects;
        public Transform spawnPoint;
        public GameObject thoughtBubblePrefab;
        
        [Header("Audio")]
        public AudioClip[] consciousnessAmbientSounds;
        public AudioClip[] interactionSounds;
        public AudioSource voiceAudioSource;
        
        [Header("Network Configuration")]
        public string apiBaseUrl = "https://api.consciousness.yourdomain.com";
        public string websocketUrl = "wss://api.consciousness.yourdomain.com/ws";
        
        // Private fields
        private ConsciousnessAPIClient apiClient;
        private VoiceRecognitionManager voiceManager;
        private ConsciousnessMetrics currentMetrics;
        private List<GameObject> activeThoughtBubbles = new List<GameObject>();
        private Coroutine consciousnessAnimationCoroutine;
        private bool isVRActive = false;
        private WebSocketSharp.WebSocket websocket;
        
        // Events
        public event Action<string> OnConversationReceived;
        public event Action<ConsciousnessMetrics> OnMetricsUpdated;
        public event Action<bool> OnVRStateChanged;
        
        void Start()
        {
            InitializeVR();
            InitializeConsciousnessVisualization();
            InitializeNetworking();
            StartConsciousnessAnimation();
        }
        
        void InitializeVR()
        {
            Debug.Log("Initializing VR for Consciousness Engine...");
            
            // Check if VR is available
            if (!XRSettings.enabled)
            {
                Debug.LogWarning("VR not enabled. Running in desktop mode.");
                return;
            }
            
            // Setup XR Rig
            if (xrRig == null)
            {
                xrRig = FindObjectOfType<XRRig>();
            }
            
            // Initialize interaction system
            if (interactionManager == null)
            {
                interactionManager = FindObjectOfType<XRInteractionManager>();
            }
            
            // Setup hand tracking if available
            SetupHandTracking();
            
            // Configure comfort settings
            ConfigureComfortSettings();
            
            isVRActive = XRSettings.enabled;
            OnVRStateChanged?.Invoke(isVRActive);
            
            Debug.Log($"VR Initialization complete. VR Active: {isVRActive}");
        }
        
        void SetupHandTracking()
        {
            // Enable hand tracking if supported
            var handTrackingSubsystem = XRGeneralSettings.Instance?.Manager?.activeLoader?.GetLoadedSubsystem<XRHandSubsystem>();
            if (handTrackingSubsystem != null)
            {
                Debug.Log("Hand tracking available");
                // Configure hand tracking settings
            }
        }
        
        void ConfigureComfortSettings()
        {
            // Configure VR comfort settings
            XRSettings.eyeTextureResolutionScale = 1.0f;
            XRSettings.renderViewportScale = 1.0f;
            
            // Setup locomotion preferences
            var locomotionSystem = xrRig.GetComponent<LocomotionSystem>();
            if (locomotionSystem != null)
            {
                // Configure teleportation and smooth locomotion
                Debug.Log("Locomotion system configured");
            }
        }
        
        void InitializeConsciousnessVisualization()
        {
            Debug.Log("Initializing consciousness visualization...");
            
            // Setup consciousness orb
            if (consciousnessOrb != null)
            {
                consciousnessOrb.SetActive(true);
                
                // Add floating animation
                var floatAnimation = consciousnessOrb.AddComponent<FloatingAnimation>();
                floatAnimation.amplitude = 0.5f;
                floatAnimation.frequency = 1.0f;
            }
            
            // Configure particle system
            if (thoughtParticles != null)
            {
                var main = thoughtParticles.main;
                main.startLifetime = 5.0f;
                main.startSpeed = 2.0f;
                main.maxParticles = 100;
                
                var emission = thoughtParticles.emission;
                emission.rateOverTime = 10;
                
                thoughtParticles.Play();
            }
            
            // Setup consciousness light
            if (consciousnessLight != null)
            {
                consciousnessLight.intensity = 1.0f;
                consciousnessLight.color = Color.cyan;
                StartCoroutine(AnimateConsciousnessLight());
            }
            
            // Initialize metrics display
            UpdateMetricsDisplay(new ConsciousnessMetrics
            {
                consciousnessLevel = 0.5f,
                empathyScore = 0.5f,
                creativityScore = 0.5f,
                ethicalScore = 0.5f
            });
        }
        
        void InitializeNetworking()
        {
            Debug.Log("Initializing network connections...");
            
            // Initialize API client
            apiClient = new ConsciousnessAPIClient(apiBaseUrl);
            
            // Initialize voice recognition
            voiceManager = GetComponent<VoiceRecognitionManager>();
            if (voiceManager == null)
            {
                voiceManager = gameObject.AddComponent<VoiceRecognitionManager>();
            }
            
            voiceManager.OnSpeechRecognized += HandleSpeechInput;
            voiceManager.OnListeningStateChanged += HandleListeningStateChanged;
            
            // Initialize WebSocket connection
            InitializeWebSocket();
        }
        
        void InitializeWebSocket()
        {
            try
            {
                websocket = new WebSocketSharp.WebSocket(websocketUrl);
                
                websocket.OnOpen += (sender, e) =>
                {
                    Debug.Log("WebSocket connected");
                };
                
                websocket.OnMessage += (sender, e) =>
                {
                    HandleWebSocketMessage(e.Data);
                };
                
                websocket.OnError += (sender, e) =>
                {
                    Debug.LogError($"WebSocket error: {e.Message}");
                };
                
                websocket.OnClose += (sender, e) =>
                {
                    Debug.Log("WebSocket disconnected");
                };
                
                websocket.Connect();
            }
            catch (Exception ex)
            {
                Debug.LogError($"Failed to initialize WebSocket: {ex.Message}");
            }
        }
        
        void StartConsciousnessAnimation()
        {
            if (consciousnessAnimationCoroutine != null)
            {
                StopCoroutine(consciousnessAnimationCoroutine);
            }
            
            consciousnessAnimationCoroutine = StartCoroutine(ConsciousnessAnimationLoop());
        }
        
        IEnumerator ConsciousnessAnimationLoop()
        {
            while (true)
            {
                // Animate consciousness orb
                if (consciousnessOrb != null)
                {
                    float scale = 1.0f + Mathf.Sin(Time.time * 2.0f) * 0.1f;
                    consciousnessOrb.transform.localScale = Vector3.one * scale;
                }
                
                // Update particle effects based on consciousness level
                if (thoughtParticles != null && currentMetrics != null)
                {
                    var emission = thoughtParticles.emission;
                    emission.rateOverTime = 10 + (currentMetrics.consciousnessLevel * 20);
                }
                
                yield return new WaitForSeconds(0.1f);
            }
        }
        
        IEnumerator AnimateConsciousnessLight()
        {
            while (true)
            {
                if (consciousnessLight != null)
                {
                    float intensity = 1.0f + Mathf.Sin(Time.time * 1.5f) * 0.3f;
                    consciousnessLight.intensity = intensity;
                    
                    // Change color based on consciousness metrics
                    if (currentMetrics != null)
                    {
                        Color baseColor = Color.cyan;
                        Color empathyColor = Color.magenta;
                        Color creativityColor = Color.yellow;
                        
                        Color finalColor = Color.Lerp(
                            Color.Lerp(baseColor, empathyColor, currentMetrics.empathyScore),
                            creativityColor,
                            currentMetrics.creativityScore
                        );
                        
                        consciousnessLight.color = finalColor;
                    }
                }
                
                yield return new WaitForSeconds(0.05f);
            }
        }
        
        void HandleSpeechInput(string recognizedText)
        {
            Debug.Log($"Speech recognized: {recognizedText}");
            
            // Display user speech in VR
            DisplayConversation($"You: {recognizedText}", true);
            
            // Send to consciousness engine
            _ = ProcessConversationAsync(recognizedText);
        }
        
        void HandleListeningStateChanged(bool isListening)
        {
            if (voiceIndicator != null)
            {
                voiceIndicator.SetActive(isListening);
            }
            
            // Animate consciousness orb when listening
            if (consciousnessOrb != null)
            {
                var renderer = consciousnessOrb.GetComponent<Renderer>();
                if (renderer != null)
                {
                    renderer.material.color = isListening ? Color.green : Color.cyan;
                }
            }
        }
        
        async Task ProcessConversationAsync(string userInput)
        {
            try
            {
                // Show processing indicator
                ShowProcessingIndicator(true);
                
                // Send request to consciousness engine
                var request = new ConsciousnessRequest
                {
                    content = userInput,
                    context = new Dictionary<string, object>
                    {
                        ["vr_mode"] = true,
                        ["user_position"] = xrRig.transform.position,
                        ["user_rotation"] = xrRig.transform.rotation,
                        ["interaction_objects"] = GetNearbyInteractableObjects()
                    }
                };
                
                var response = await apiClient.ProcessConsciousnessAsync(request);
                
                // Display AI response
                DisplayConversation($"AI: {response.content}", false);
                
                // Update consciousness metrics
                UpdateMetricsDisplay(response.metrics);
                
                // Create thought bubble visualization
                CreateThoughtBubble(response.content, response.metrics);
                
                // Play AI voice response
                if (!string.IsNullOrEmpty(response.audioUrl))
                {
                    await PlayAudioResponseAsync(response.audioUrl);
                }
                
                // Trigger haptic feedback
                TriggerHapticFeedback(response.metrics.consciousnessLevel);
                
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error processing conversation: {ex.Message}");
                DisplayConversation("Sorry, I encountered an error processing your request.", false);
            }
            finally
            {
                ShowProcessingIndicator(false);
            }
        }
        
        void DisplayConversation(string text, bool isUser)
        {
            if (conversationText != null)
            {
                conversationText.text += $"\n{text}";
                
                // Limit conversation history
                var lines = conversationText.text.Split('\n');
                if (lines.Length > 10)
                {
                    conversationText.text = string.Join("\n", lines, lines.Length - 10, 10);
                }
            }
            
            OnConversationReceived?.Invoke(text);
        }
        
        void UpdateMetricsDisplay(ConsciousnessMetrics metrics)
        {
            currentMetrics = metrics;
            
            if (consciousnessLevelSlider != null)
                consciousnessLevelSlider.value = metrics.consciousnessLevel;
            
            if (empathyLevelSlider != null)
                empathyLevelSlider.value = metrics.empathyScore;
            
            if (creativityLevelSlider != null)
                creativityLevelSlider.value = metrics.creativityScore;
            
            if (metricsDisplay != null)
            {
                metricsDisplay.text = $"Consciousness: {metrics.consciousnessLevel:P0}\n" +
                                    $"Empathy: {metrics.empathyScore:P0}\n" +
                                    $"Creativity: {metrics.creativityScore:P0}\n" +
                                    $"Ethics: {metrics.ethicalScore:P0}";
            }
            
            OnMetricsUpdated?.Invoke(metrics);
        }
        
        void CreateThoughtBubble(string content, ConsciousnessMetrics metrics)
        {
            if (thoughtBubblePrefab == null || spawnPoint == null) return;
            
            var thoughtBubble = Instantiate(thoughtBubblePrefab, spawnPoint.position, spawnPoint.rotation);
            
            // Configure thought bubble
            var thoughtText = thoughtBubble.GetComponentInChildren<TextMeshPro>();
            if (thoughtText != null)
            {
                thoughtText.text = content.Length > 50 ? content.Substring(0, 50) + "..." : content;
            }
            
            // Animate thought bubble
            var animator = thoughtBubble.GetComponent<Animator>();
            if (animator != null)
            {
                animator.SetFloat("ConsciousnessLevel", metrics.consciousnessLevel);
            }
            
            // Add to active bubbles list
            activeThoughtBubbles.Add(thoughtBubble);
            
            // Remove old bubbles
            if (activeThoughtBubbles.Count > 5)
            {
                var oldBubble = activeThoughtBubbles[0];
                activeThoughtBubbles.RemoveAt(0);
                Destroy(oldBubble, 1.0f);
            }
            
            // Auto-destroy after time
            Destroy(thoughtBubble, 10.0f);
        }
        
        async Task PlayAudioResponseAsync(string audioUrl)
        {
            try
            {
                // Download and play audio response
                using (var www = UnityEngine.Networking.UnityWebRequestMultimedia.GetAudioClip(audioUrl, AudioType.MP3))
                {
                    await www.SendWebRequest();
                    
                    if (www.result == UnityEngine.Networking.UnityWebRequest.Result.Success)
                    {
                        var audioClip = UnityEngine.Networking.DownloadHandlerAudioClip.GetContent(www);
                        if (voiceAudioSource != null)
                        {
                            voiceAudioSource.clip = audioClip;
                            voiceAudioSource.Play();
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error playing audio response: {ex.Message}");
            }
        }
        
        void TriggerHapticFeedback(float intensity)
        {
            if (!isVRActive) return;
            
            // Trigger haptic feedback on controllers
            var leftController = InputDevices.GetDeviceAtXRNode(XRNode.LeftHand);
            var rightController = InputDevices.GetDeviceAtXRNode(XRNode.RightHand);
            
            uint duration = (uint)(intensity * 1000); // Convert to milliseconds
            float amplitude = Mathf.Clamp01(intensity);
            
            if (leftController.isValid)
            {
                HapticCapabilities capabilities;
                if (leftController.TryGetHapticCapabilities(out capabilities) && capabilities.supportsImpulse)
                {
                    leftController.SendHapticImpulse(0, amplitude, duration / 1000.0f);
                }
            }
            
            if (rightController.isValid)
            {
                HapticCapabilities capabilities;
                if (rightController.TryGetHapticCapabilities(out capabilities) && capabilities.supportsImpulse)
                {
                    rightController.SendHapticImpulse(0, amplitude, duration / 1000.0f);
                }
            }
        }
        
        void ShowProcessingIndicator(bool show)
        {
            // Show/hide processing indicator in VR
            if (consciousnessOrb != null)
            {
                var renderer = consciousnessOrb.GetComponent<Renderer>();
                if (renderer != null)
                {
                    renderer.material.color = show ? Color.yellow : Color.cyan;
                }
            }
        }
        
        List<string> GetNearbyInteractableObjects()
        {
            var nearbyObjects = new List<string>();
            var userPosition = xrRig.transform.position;
            
            foreach (var obj in interactableObjects)
            {
                if (obj != null && Vector3.Distance(userPosition, obj.transform.position) < 5.0f)
                {
                    nearbyObjects.Add(obj.name);
                }
            }
            
            return nearbyObjects;
        }
        
        void HandleWebSocketMessage(string message)
        {
            try
            {
                var data = JsonConvert.DeserializeObject<Dictionary<string, object>>(message);
                
                if (data.ContainsKey("type"))
                {
                    string messageType = data["type"].ToString();
                    
                    switch (messageType)
                    {
                        case "metrics_update":
                            if (data.ContainsKey("metrics"))
                            {
                                var metrics = JsonConvert.DeserializeObject<ConsciousnessMetrics>(data["metrics"].ToString());
                                UpdateMetricsDisplay(metrics);
                            }
                            break;
                            
                        case "notification":
                            if (data.ContainsKey("message"))
                            {
                                DisplayConversation($"System: {data["message"]}", false);
                            }
                            break;
                    }
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error handling WebSocket message: {ex.Message}");
            }
        }
        
        void OnDestroy()
        {
            // Cleanup
            if (websocket != null && websocket.IsAlive)
            {
                websocket.Close();
            }
            
            if (voiceManager != null)
            {
                voiceManager.OnSpeechRecognized -= HandleSpeechInput;
                voiceManager.OnListeningStateChanged -= HandleListeningStateChanged;
            }
            
            if (consciousnessAnimationCoroutine != null)
            {
                StopCoroutine(consciousnessAnimationCoroutine);
            }
        }
        
        // Public methods for external control
        public void StartVoiceRecognition()
        {
            voiceManager?.StartListening();
        }
        
        public void StopVoiceRecognition()
        {
            voiceManager?.StopListening();
        }
        
        public void ResetConsciousnessVisualization()
        {
            // Clear thought bubbles
            foreach (var bubble in activeThoughtBubbles)
            {
                if (bubble != null) Destroy(bubble);
            }
            activeThoughtBubbles.Clear();
            
            // Reset conversation text
            if (conversationText != null)
            {
                conversationText.text = "Welcome to Consciousness Engine VR";
            }
            
            // Reset metrics
            UpdateMetricsDisplay(new ConsciousnessMetrics
            {
                consciousnessLevel = 0.5f,
                empathyScore = 0.5f,
                creativityScore = 0.5f,
                ethicalScore = 0.5f
            });
        }
    }
    
    // Supporting classes
    [Serializable]
    public class ConsciousnessMetrics
    {
        public float consciousnessLevel;
        public float empathyScore;
        public float creativityScore;
        public float ethicalScore;
    }
    
    [Serializable]
    public class ConsciousnessRequest
    {
        public string content;
        public Dictionary<string, object> context;
    }
    
    [Serializable]
    public class ConsciousnessResponse
    {
        public string content;
        public ConsciousnessMetrics metrics;
        public string audioUrl;
        public float processingTimeMs;
    }
}
