package com.mazerion.ui.theme

import android.app.Activity
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat

private val DarkColorScheme = darkColorScheme(
    primary = HoneyGold,
    onPrimary = DeepBrown,
    primaryContainer = HoneyGoldDark,
    onPrimaryContainer = CreamWhite,
    secondary = Amber,
    onSecondary = DeepBrown,
    secondaryContainer = AmberDark,
    onSecondaryContainer = CreamWhite,
    tertiary = LightBrown,
    background = DeepBrown,
    onBackground = CreamWhite,
    surface = LightBrown,
    onSurface = CreamWhite,
    error = Color(0xFFCF6679),
    onError = Color(0xFF000000)
)

private val LightColorScheme = lightColorScheme(
    primary = HoneyGold,
    onPrimary = DeepBrown,
    primaryContainer = HoneyGoldLight,
    onPrimaryContainer = DeepBrown,
    secondary = Amber,
    onSecondary = DeepBrown,
    secondaryContainer = AmberLight,
    onSecondaryContainer = DeepBrown,
    tertiary = LightBrown,
    background = CreamWhite,
    onBackground = DeepBrown,
    surface = CreamWhite,
    onSurface = DeepBrown,
    error = Color(0xFFB00020),
    onError = Color(0xFFFFFFFF)
)

@Composable
fun MazerionTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }

    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = (view.context as Activity).window
            window.statusBarColor = colorScheme.primary.toArgb()
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
        }
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}
