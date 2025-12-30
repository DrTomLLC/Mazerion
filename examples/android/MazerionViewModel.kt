package com.drtomllc.mazerion.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import mazerion.*

data class CalculatorState(
    val calculators: List<CalculatorInfo> = emptyList(),
    val categories: Map<String, Int> = emptyMap(),
    val result: CalcResult? = null,
    val error: String? = null,
    val isLoading: Boolean = false
)

class MazerionViewModel : ViewModel() {
    private val _state = MutableStateFlow(CalculatorState())
    val state: StateFlow<CalculatorState> = _state

    init {
        loadCalculators()
    }

    private fun loadCalculators() {
        viewModelScope.launch {
            _state.value = _state.value.copy(isLoading = true)
            try {
                val calculators = listCalculators()
                val categoryMap = getCategories()
                val categories = categoryMap.entries.associate {
                    it.category to it.count.toInt()
                }

                _state.value = _state.value.copy(
                    calculators = calculators,
                    categories = categories,
                    isLoading = false,
                    error = null
                )
            } catch (e: MazerionException) {
                _state.value = _state.value.copy(
                    error = e.message,
                    isLoading = false
                )
            }
        }
    }

    fun calculateABV(og: String, fg: String) {
        viewModelScope.launch {
            _state.value = _state.value.copy(isLoading = true)
            try {
                val params = listOf(
                    CalcParam("og", og),
                    CalcParam("fg", fg)
                )
                val result = executeCalculator("abv", params)
                _state.value = _state.value.copy(
                    result = result,
                    isLoading = false,
                    error = null
                )
            } catch (e: MazerionException) {
                _state.value = _state.value.copy(
                    error = e.message,
                    isLoading = false
                )
            }
        }
    }

    fun batchCalculate(requests: List<BatchCalculatorRequest>) {
        viewModelScope.launch {
            _state.value = _state.value.copy(isLoading = true)
            try {
                val results = executeBatch(requests)
                // Handle batch results
                _state.value = _state.value.copy(
                    isLoading = false,
                    error = null
                )
            } catch (e: MazerionException) {
                _state.value = _state.value.copy(
                    error = e.message,
                    isLoading = false
                )
            }
        }
    }
}