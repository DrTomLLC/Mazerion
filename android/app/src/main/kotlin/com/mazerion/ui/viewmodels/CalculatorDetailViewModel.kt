package com.mazerion.ui.viewmodels

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.mazerion.data.MazerionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import mazerion.CalcParam
import mazerion.CalcResult

data class CalculatorDetailUiState(
    val calculatorId: String = "",
    val calculatorName: String = "",
    val result: CalcResult? = null,
    val isLoading: Boolean = false,
    val error: String? = null
)

class CalculatorDetailViewModel(
    private val calculatorId: String,
    private val repository: MazerionRepository = MazerionRepository()
) : ViewModel() {

    private val _uiState = MutableStateFlow(CalculatorDetailUiState(calculatorId = calculatorId))
    val uiState: StateFlow<CalculatorDetailUiState> = _uiState.asStateFlow()

    private val _params = MutableStateFlow<Map<String, String>>(emptyMap())
    val params: StateFlow<Map<String, String>> = _params.asStateFlow()

    init {
        loadCalculatorInfo()
    }

    private fun loadCalculatorInfo() {
        viewModelScope.launch {
            repository.listCalculators()
                .onSuccess { calculators ->
                    val calculator = calculators.find { it.id == calculatorId }
                    if (calculator != null) {
                        _uiState.value = _uiState.value.copy(
                            calculatorName = calculator.name
                        )
                    }
                }
        }
    }

    fun updateParam(key: String, value: String) {
        _params.value = _params.value.toMutableMap().apply {
            put(key, value)
        }
    }

    fun calculate() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true, error = null)

            val paramsList = _params.value.map { (key, value) ->
                CalcParam(key, value)
            }

            repository.executeCalculator(calculatorId, paramsList)
                .onSuccess { result ->
                    _uiState.value = _uiState.value.copy(
                        result = result,
                        isLoading = false
                    )
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        error = error.message ?: "Calculation failed",
                        isLoading = false
                    )
                }
        }
    }

    class Factory(private val calculatorId: String) : ViewModelProvider.Factory {
        @Suppress("UNCHECKED_CAST")
        override fun <T : ViewModel> create(modelClass: Class<T>): T {
            return CalculatorDetailViewModel(calculatorId) as T
        }
    }
}